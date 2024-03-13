use tspf::TspBuilder;

use super::{SolvingOutput, SolvingTrait};
use std::{path::Path, process::exit};

pub struct FileSolver {
    pub solution_file_dir: String,
}

impl SolvingTrait for FileSolver {
    fn solve(&self, path: &str, _transform_only: Option<bool>) -> SolvingOutput {
        let file_name = Path::new(path).file_stem().unwrap().to_str().unwrap();

        let file_path = format!("{}/{}.sol", self.solution_file_dir, file_name);

        println!("file path: {}", file_path);

        if let Ok(tour) = TspBuilder::parse_path(&file_path[..]) {
            SolvingOutput::new(tour.tours().clone())
        } else {
            println!("Failed to open tour file");
            exit(1)
        }
    }
}
