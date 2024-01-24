use std::{
    fmt, fs,
    io::{BufRead, BufReader, Error},
    path::PathBuf,
    process::{exit, Command, Stdio},
    time::SystemTime,
};

use crate::logic::util::tsp::Distancing;

use super::SolvingTrait;
use std::io::Write;
use tspf::{Tsp, TspBuilder};

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

        let cols = if rows > 0 { self.0[0].len() } else { 0 };

        let values = self
            .0
            .iter()
            .flatten()
            .filter(|x| **x != 0f64)
            .collect::<Vec<&f64>>()
            .len();

        writeln!(writer, "{rows} {cols} {values}")?;

        for (i, row) in self.0.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                let i = i + 1;
                let j = j + 1;
                if *value != 0f64 {
                    writeln!(writer, "{i} {j} {value}")?;
                }
            }
        }
        Ok(())
    }
}

impl From<&Tsp> for COOrdinate {
    fn from(tsp: &Tsp) -> Self {
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
                distances[u - 1][i - 1] = dist;
            }
        }

        let a = (dim as f64) * max_distance;
        let b = 1f64;

        let mut matrix = vec![vec![0f64; dim * dim]; dim * dim];

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

        for i in 1..dim * dim {
            for j in 0..=i - 1 {
                matrix[j][i] += matrix[i][j];
                matrix[i][j] = 0f64;
            }
        }
        COOrdinate(matrix)
    }
}

pub enum HybridTspSolverType {
    Simulated,
    LeapHybrid,
    QbSolv,
    Direct,
}

impl fmt::Display for HybridTspSolverType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HybridTspSolverType::Simulated => write!(f, "sim"),
            HybridTspSolverType::LeapHybrid => write!(f, "hybrid"),
            HybridTspSolverType::QbSolv => write!(f, "qbsolv"),
            HybridTspSolverType::Direct => write!(f, "direct"),
        }
    }
}

pub struct HybridTspSolver {
    pub quantum_type: HybridTspSolverType,
}

impl SolvingTrait for HybridTspSolver {
    fn solve(&self, path: &str, transform_only: Option<bool>) -> super::SolvingOutput {
        let tsp = match TspBuilder::parse_path(path) {
            Ok(tsp) => tsp,
            Err(e) => {
                println!("problems with parsing subproblem {e}");
                exit(1)
            }
        };

        let before_transform_time = SystemTime::now();
        println!("hybrid qubo transform {path} start");

        let coo = COOrdinate::from(&tsp);

        let srcdir = PathBuf::from(path);
        let abs_path = fs::canonicalize(srcdir).unwrap();
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

        let abs_coo_file_name = format!("{}{}", file_name, "coo");

        let mut file = match std::fs::File::create(&abs_coo_file_name) {
            Ok(file) => file,
            Err(e) => {
                println!("Problem opening coordinate file {e}");
                exit(1)
            }
        };

        if let Err(e) = coo.write_coordinate(&mut file) {
            println!("Problem writing coordinate file {e}");
            exit(1)
        }

        let after_transform_time = SystemTime::now()
            .duration_since(before_transform_time)
            .unwrap()
            .as_secs_f32();
        println!("hybrid qubo transform {path} end: {after_transform_time}");

        if let Some(true) = transform_only {
            return vec![vec![]];
        }

        let output_file_name = format!("{}{}", file_name, "bin");

        let mut cmd = Command::new("poetry")
            .current_dir("./python/qubo_solver")
            .arg("run")
            .arg("python")
            .arg("/Users/lucas/workspace/uni/bachelor/pipeline/python/qubo_solver/src/main.py")
            .arg(abs_coo_file_name)
            .arg(self.quantum_type.to_string())
            .arg("--output-file")
            .arg(&output_file_name)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        {
            let stdout = cmd.stdout.as_mut().unwrap();
            let stdout_reader = BufReader::new(stdout);
            let stdout_lines = stdout_reader.lines();

            for line in stdout_lines {
                println!("QUBO Solver: {}", line.unwrap());
            }
        }

        let before_post_transform_time = SystemTime::now();
        println!("hybrid post transform {path} start");
        let result = match fs::read_to_string(&output_file_name) {
            Ok(res) => res,
            Err(err) => {
                println!("Problem opening file \"{output_file_name}\": {err}");
                exit(1)
            }
        };

        let mut places: Vec<(usize, usize)> = result
            .split_whitespace()
            .collect::<Vec<&str>>()
            .chunks(tsp.dim())
            .map(|chunk| chunk.iter().position(|x| *x == "1").unwrap())
            .enumerate()
            .map(|(point, place)| (place, point + 1))
            .collect();

        places.sort_by(|(a_place, _), (b_place, _)| a_place.cmp(b_place));

        let after_post_transform_time = SystemTime::now()
            .duration_since(before_post_transform_time)
            .unwrap()
            .as_secs_f32();
        println!("hybrid post transform {path} end: {after_post_transform_time}");

        vec![places.iter().map(|(_, point)| *point).collect()]
    }
}
