use super::{SolvingOutput, SolvingTrait};
use std::fs::File;
use std::io::BufReader;
use std::process::exit;
use std::sync::Arc;

use vrp_scientific::{
    core::{
        rosomaxa::prelude::TelemetryMode,
        solver::{create_default_config_builder, Solver},
        utils::Environment,
    },
    tsplib::*,
};

pub struct RustVrpSolver;

impl SolvingTrait for RustVrpSolver {
    fn solve(&self, path: &str, _transform_only: Option<bool>) -> SolvingOutput {
        let vrp_file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("Something opening the temporary path of a vrp: \n{e}");
                exit(1)
            }
        };
        let reader = BufReader::new(vrp_file);

        let arc_problem = {
            let problem = match reader.read_tsplib(false) {
                Ok(problem) => problem,
                Err(error) => {
                    println!("Something went wrong parsing a sub VRP: \n{error}");
                    exit(1);
                }
            };
            Arc::new(problem)
        };

        let arc_env = Arc::new(Environment::default());

        let config =
            match create_default_config_builder(arc_problem.clone(), arc_env, TelemetryMode::None)
                .build()
            {
                Ok(config) => config,
                Err(e) => {
                    println!("Something went wrong building the config: \n{e}");
                    exit(1)
                }
            };

        let solver = Solver::new(arc_problem.clone(), config);
        let (solution, _cost) = match solver.solve() {
            Ok((sol, cost, _)) => (sol, cost),
            Err(e) => {
                println!("Something went wrong solving a partial vrp: \n{e}");
                exit(1)
            }
        };

        solution
            .routes
            .iter()
            .map(|r| r.tour.all_activities().map(|a| a.place.location).collect())
            .collect()
    }
}
