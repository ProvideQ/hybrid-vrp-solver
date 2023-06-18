mod args;
mod error_code;

mod logic {
    pub mod solver;
}

use std::process::exit;

use args::{VRPCommand, VRPSolverArgs};
use clap::Parser;
use error_code::ExitCode;
use logic::solver::{KNNClustering, VrpSolver};
use tspf::{TspBuilder, TspKind};

fn main() {
    let args = VRPSolverArgs::parse();

    match args.command {
        VRPCommand::Solve(subcommandargs) => {
            let path = subcommandargs.path;
            println!("path: {}", path);

            let vrp_instance = match TspBuilder::parse_path(path) {
                Ok(instance) => instance,
                Err(e) => {
                    println!("Problems reading the VRP-Instance: {}", e);
                    exit(ExitCode::ReadProblems as i32);
                }
            };
            if vrp_instance.kind() != TspKind::Cvrp {
                println!(
                    "Invalid TSPLIB instance type {}. (supported is CVRP)",
                    vrp_instance.kind().to_string().to_uppercase()
                );
                exit(ExitCode::WrongTspType as i32);
            }

            println!("name: {}", vrp_instance.name());
            println!("type: {}", vrp_instance.kind());

            println!("solve");
            let solver = VrpSolver {
                cluster_strat: Box::new(KNNClustering { count: 20 }),
            };

            println!("result {}", solver.solve(vrp_instance));
        }
    }
}
