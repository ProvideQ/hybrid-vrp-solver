use std::{
    fmt::{self, Debug},
    io::{Error, Write},
};

use tspf::Tsp;

use crate::logic::util;

pub struct SolvingOutput(Vec<Vec<usize>>);

impl SolvingOutput {
    pub fn new(output: Vec<Vec<usize>>) -> Self {
        Self(output)
    }

    pub fn output(&self) -> &Vec<Vec<usize>> {
        &self.0
    }
}

impl From<SolvingOutput> for Vec<Vec<usize>> {
    fn from(solution: SolvingOutput) -> Vec<Vec<usize>> {
        solution.0
    }
}

impl Debug for SolvingOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SolvingOutput({:?})", self.0)
    }
}

pub trait VRPTourWriter {
    /// Writes tours in tour format
    fn write_tours<W: Write>(&self, writer: &mut W) -> Result<(), Error>;
}

impl VRPTourWriter for (&Tsp, &SolvingOutput) {
    fn write_tours<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        let vrp = self.0;
        let solution = self.1.output().clone();

        let name = vrp.name();
        let dim = vrp.dim();
        let len = util::tsp::calculate_solution_score(vrp, &solution);

        writeln!(writer, "NAME : {name} solved with length {len}")?;
        writeln!(writer, "TYPE : TOUR")?;
        writeln!(writer, "DIMENSION : {dim}")?;
        writeln!(writer, "TOUR_SECTION")?;
        for path in solution {
            let path = path
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            writeln!(writer, "{path} -1")?;
        }
        write!(writer, "-1\nEOF")?;

        Ok(())
    }
}

pub trait SolvingTrait {
    fn solve(&self, path: &str, transform_only: Option<bool>) -> SolvingOutput;
}
