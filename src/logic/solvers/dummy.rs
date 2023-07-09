use std::process::exit;

use tspf::TspBuilder;

use super::SolvingTrait;

pub struct DummySolver;

impl SolvingTrait for DummySolver {
    fn solve(&self, path: &str) -> super::SolvingOutput {
        let vrp = match TspBuilder::parse_path(path) {
            Ok(tsp) => tsp,
            Err(_) => {
                println!("something went wrong in the DummySolver");
                exit(1)
            }
        };

        if vrp.node_coords().into_iter().any(|(id, p)| *id > vrp.dim()) {
            println!("DummySolver: There is a point with a greater id than there are points");
            exit(1)
        }

        vec![(0..(vrp.dim() - 1)).collect()]
    }
}
