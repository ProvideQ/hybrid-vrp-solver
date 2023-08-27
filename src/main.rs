mod args;
mod error_code;
mod logic;

use std::env;

use args::{ClusterOption, SolverOption, VRPCommand, VRPSolverArgs};
use clap::Parser;

use logic::clustering::{ClusterTspClustering, KNNClustering};
use logic::solver::VrpSolver;
use logic::solvers::{HybridTspSolver, LKHSolver, SolvingTrait};

impl From<SolverOption> for Box<dyn SolvingTrait> {
    fn from(val: SolverOption) -> Self {
        match val {
            SolverOption::Lkh => Box::new(LKHSolver {
                binary: String::from("./bin/LKH"),
            }),
            SolverOption::Simulated => Box::new(HybridTspSolver {
                quantum_type: logic::solvers::HybridTspSolverType::Simulated,
            }),
            SolverOption::LeapHybrid => Box::new(HybridTspSolver {
                quantum_type: logic::solvers::HybridTspSolverType::LeapHybrid,
            }),
            SolverOption::QbSolv => Box::new(HybridTspSolver {
                quantum_type: logic::solvers::HybridTspSolverType::QbSolv,
            }),
            SolverOption::Direct => Box::new(HybridTspSolver {
                quantum_type: logic::solvers::HybridTspSolverType::Direct,
            }),
        }
    }
}

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
                cluster_strat: match subcommandargs.cluster {
                    ClusterOption::Knn => Box::new(KNNClustering {
                        count: subcommandargs.cluster_number,
                    }),
                    ClusterOption::Tsp => Box::new(ClusterTspClustering {}),
                },
                solving_strat: Box::<dyn SolvingTrait>::from(subcommandargs.solver),
                build_dir: Some(subcommandargs.build_dir),
            };

            println!("result {:?}", solver.solve(&path[..]));
        }
    }
}
