mod args;
mod error_code;
mod logic;

use std::env;

use args::{ClusterOption, SolverOption, VRPCommand, VRPSolverArgs};
use clap::Parser;

use logic::clustering::{ClusterTspClustering, KMeansClustering};
use logic::solver::VrpSolver;
use logic::solvers::{DummySolver, HybridTspSolver, LKHSolver, SolvingTrait};
use tspf::{TspBuilder, TspKind};

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
            let solver = if let SolverOption::Direct = subcommandargs.solver {
                VrpSolver {
                    cluster_strat: match subcommandargs.cluster {
                        ClusterOption::Kmeans => Box::new(KMeansClustering {
                            count: subcommandargs.cluster_number,
                        }),
                        ClusterOption::Tsp => Box::new(ClusterTspClustering {}),
                        ClusterOption::ClusterFromFile => {
                            Box::new(logic::clustering::FileClustering {
                                map_file_path: subcommandargs.cluster_file,
                            })
                        }
                    },
                    solving_strat: Box::new(VrpSolver {
                        cluster_strat: Box::new(ClusterTspClustering {}),
                        solving_strat: Box::<dyn SolvingTrait>::from(subcommandargs.solver),
                        build_dir: Some(subcommandargs.build_dir.clone()),
                    }),
                    build_dir: Some(subcommandargs.build_dir),
                }
            } else {
                VrpSolver {
                    cluster_strat: match subcommandargs.cluster {
                        ClusterOption::Kmeans => Box::new(KMeansClustering {
                            count: subcommandargs.cluster_number,
                        }),
                        ClusterOption::Tsp => Box::new(ClusterTspClustering {}),
                        ClusterOption::ClusterFromFile => {
                            Box::new(logic::clustering::FileClustering {
                                map_file_path: subcommandargs.cluster_file,
                            })
                        }
                    },
                    solving_strat: Box::<dyn SolvingTrait>::from(subcommandargs.solver),
                    build_dir: Some(subcommandargs.build_dir),
                }
            };

            println!("result {:?}", solver.solve(&path[..], Option::None));
        }
        VRPCommand::Partial(partial) => match partial.subcommand {
            args::PartialSolveSubCommand::Cluster(cluster_opt) => {
                let solver = VrpSolver {
                    cluster_strat: match cluster_opt.cluster {
                        ClusterOption::Kmeans => Box::new(KMeansClustering {
                            count: cluster_opt.cluster_number,
                        }),
                        ClusterOption::Tsp => Box::new(ClusterTspClustering {}),
                        ClusterOption::ClusterFromFile => {
                            Box::new(logic::clustering::FileClustering {
                                map_file_path: cluster_opt.cluster_file,
                            })
                        }
                    },
                    solving_strat: Box::new(DummySolver {}),
                    build_dir: Some(cluster_opt.build_dir),
                };

                let vrp = TspBuilder::parse_path(&cluster_opt.path[..]).unwrap();

                if vrp.kind() != TspKind::Cvrp {
                    panic!(
                        "Invalid TSPLIB instance type {}. (supported is CVRP)",
                        vrp.kind().to_string().to_uppercase()
                    );
                }

                solver.partial_cluster(&vrp);
            }
            args::PartialSolveSubCommand::Solve(solve_opt) => {
                let solver = Box::<dyn SolvingTrait>::from(solve_opt.solver);

                solver.solve(&solve_opt.path[..], Option::Some(solve_opt.transform_only));
            }
        },
    }
}
