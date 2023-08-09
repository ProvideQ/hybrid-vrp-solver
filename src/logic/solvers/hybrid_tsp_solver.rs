use std::{process::exit, borrow::BorrowMut, io::Error, path::PathBuf, fs};

use tspf::{TspBuilder, Tsp};
use std::io::Write;
use super::SolvingTrait;


struct COOrdinate(Vec<Vec<f64>>);
/// A trait to write tsplib95 solution.
pub trait COOrdinateWriter {
    /// Writes coordinate matrix to file.
    fn write_coordinate<W: Write>(&self, writer: &mut W) -> Result<(), Error>;
}

impl COOrdinateWriter for COOrdinate {
    fn write_coordinate<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writeln!(writer, "%%MatrixMarket matrix coordinate real general")?;

        let rows = self.0.len();

        let cols = if rows > 0 {
            self.0[0].len()
        } else {
            0
        };

        let values = self.0.iter().flatten().filter(|x| **x != 0f64).collect::<Vec<&f64>>().len();

        writeln!(writer, "{rows} {cols} {values}")?;

        for (i, row) in self.0.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                let i = i+1;
                let j = j+1;
                if *value != 0f64 {
                    writeln!(writer, "{i} {j} {value}")?;
                }
            }
        }
        Ok(())
    }
}

trait Distancing<T> {
    fn distance(&self, a: usize, b: usize) -> Option<T>;
}

impl Distancing<f64> for Tsp {
    fn distance(&self, a: usize, b: usize) -> Option<f64> {
        let a = match self.node_coords().get(&a) {
            Some(p) => p.pos(),
            None => {
                return None;
            }
        };
        let b = match self.node_coords().get(&b) {
            Some(p) => p.pos(),
            None => {
                return None;
            }
        };
        if a.len() != b.len() {
            return None;
        }
        Some(
            a.iter()
                .zip(b.iter())
                .map(|(av, bv)| (av - bv) * (av - bv))
                .reduce(|a, b| a + b)
                .unwrap()
                .sqrt(),
        )
    }
}

pub struct HybridTspSolver {
}

impl SolvingTrait for HybridTspSolver {
    fn solve(&self, path: &str) -> super::SolvingOutput {
        let tsp = match TspBuilder::parse_path(path) {
            Ok(tsp) => tsp,
            Err(e) => {
                println!("problems with parsing subproblem {e}");
                exit(1)
            }
        };

        if tsp.demands().values().sum::<f64>() > tsp.capacity() {
            println!("CVRP subproblem can't be solved with this");
            exit(1);
        }

        let dim = tsp.dim();

        let mut max_distance: f64 = 0f64; 
        let mut distances = vec![vec![0f64; dim]; dim];

        for u in 1..=dim {
            for i in 1..=dim {
                let dist = tsp.distance(u, i).unwrap();
                if dist > max_distance {
                    max_distance = dist;
                }
                distances[u-1][i-1] = dist;

            }
        }

        let a = (dim as f64) * max_distance;
        let b = 1f64;

        let mut matrix = vec![vec![0f64; dim*dim]; dim*dim];

        for i in 0..dim {
            for jin in 0..dim {
                for jout in 0..dim {
                    matrix[i * dim + jin][i * dim + jout] += a;
                }
            }
            for j in 0..dim {
                matrix[i * dim + j][i * dim + j] += -2f64 * a;
            }
        }

        for j in 0..dim {
            for iin in 0..dim {
                for iout in 0..dim {
                    matrix[iin * dim + j][iout * dim + j] += a;
                }
            }
            for i in 0..dim {
                matrix[i * dim + j][i * dim + j] += -2f64 * a;
            }
        }

        for u in 0..dim {
            for i in 0..dim {
                for j in 0..dim - 1 {
                    matrix[u * dim + j][i * dim + j + 1] += b * distances[u][i];
                }
            }
        }

        for i in 1..dim*dim {
            for j in 0..=i-1 {
                matrix[j][i] += matrix[i][j];
                matrix[i][j] = 0f64;
            }
        }

        let srcdir = PathBuf::from(path);
        let abs_path = fs::canonicalize(&srcdir).unwrap();
        let abs_path = abs_path.to_str().unwrap();
        let file_name = abs_path
            .clone()
            .split_inclusive('.')
            .collect::<Vec<&str>>()
            .split_last()
            .unwrap()
            .1
            .iter()
            .fold(String::from(""), |x, y| x + y);

        let mut file = match std::fs::File::create(format!("{}{}", file_name, "coo")) {
            Ok(file) => file,
            Err(e) => {
                println!("Problem opening coordinate file {e}");
                exit(1)
            }
        };

        match COOrdinate(matrix).write_coordinate(&mut file) {
            Err(e) => {
                println!("Problem writing coordinate file {e}");
                exit(1)
            }
            _ => ()
        };

        return vec![];
        
    }
}