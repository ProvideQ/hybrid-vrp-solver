use std::{
    fs,
    io::{stdout, Write},
    path::PathBuf,
    process::Command,
    str,
};

use super::{DummySolver, SolvingTrait};

pub struct LKHSolver {
    pub binary: String,
}

impl SolvingTrait for LKHSolver {
    fn solve(&self, path: &str) -> super::SolvingOutput {
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

        let output_file_path = format!("{}{}", file_name, "tour");

        println!("{}", output_file_path);

        let output = Command::new("poetry")
            .current_dir("./python/lkh-interface")
            .arg("run")
            .arg("python")
            .arg("/Users/lucas/workspace/uni/bachelor/pipeline/python/lkh-interface/src/main.py")
            .arg(abs_path)
            .arg("--output-file")
            .arg(output_file_path)
            .output()
            .unwrap();

        println!("status: {}", output.status);
        stdout().write_all(&output.stdout).unwrap();
        vec![]
    }
}
