use std::{
    fs,
    path::PathBuf,
    process::{exit, Command},
    str,
};

use tspf::TspBuilder;

use super::SolvingTrait;

pub struct LKHSolver {
    pub binary: String,
}

impl SolvingTrait for LKHSolver {
    fn solve(&self, path: &str) -> super::SolvingOutput {
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

        let output_file_path = format!("{}{}", file_name, "tour");

        Command::new("poetry")
            .current_dir("./python/lkh-interface")
            .arg("run")
            .arg("python")
            .arg("/Users/lucas/workspace/uni/bachelor/pipeline/python/lkh-interface/src/main.py")
            .arg(abs_path)
            .arg("--output-file")
            .arg(&output_file_path)
            .output()
            .unwrap();

        if let Ok(tour) = TspBuilder::parse_path(&output_file_path[..]) {
            tour.tours().clone()
        } else {
            println!("Failed to open tour file");
            exit(1)
        }
    }
}
