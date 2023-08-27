mod args;
mod error_code;
mod logic;

use std::env;

use args::{VRPCommand, VRPSolverArgs};
use clap::Parser;
use logic::solver::VrpSolver;
use logic::solvers::{HybridTspSolver, SolvingTrait};

use crate::logic::clustering::{ClusterTspClustering, KNNClustering};
use crate::logic::solvers::LKHSolver;

fn main() {
    let args = VRPSolverArgs::parse();

    match args.command {
        VRPCommand::Solve(subcommandargs) => {
            let path = subcommandargs.path;
            match env::current_dir() {
                Ok(path) => println!("working dir: {}", path.display()),
                Err(_) => println!("errors finding the working dir"),
            }
            println!("path: {}", path);

            println!("solve");
            let solver = VrpSolver {
                cluster_strat: Box::new(KNNClustering { count: 10 }),
                solving_strat: Box::new(HybridTspSolver {
                    quantum_type: logic::solvers::HybridTspSolverType::Direct,
                }),
            };

            println!("result {:?}", solver.solve(&path[..]));
        }
    }
}
