mod args;
mod error_code;
mod logic;

use std::env;

use args::{VRPCommand, VRPSolverArgs};
use clap::Parser;
use logic::clustering::KNNClustering;
use logic::solver::VrpSolver;
use logic::solvers::{RustVrpSolver, SolvingTrait};

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
                cluster_strat: Box::new(KNNClustering { count: 20 }),
                solving_strat: Box::new(RustVrpSolver {}),
            };

            println!("result {:?}", solver.solve(&path[..]));
        }
    }
}
