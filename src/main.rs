mod args;
mod error_code;
mod logic;

use std::env;
use std::path::Path;
use std::process::exit;

use args::{
    ClusterOption, OnlySolveCommand, SolveCommand, SolverOption, VRPCommand, VRPSolverArgs,
};
use clap::Parser;

use logic::clustering::{ClusterTspClustering, KMeansClustering};
use logic::solver::VrpSolver;
use logic::solvers::{DummySolver, FileSolver, HybridTspSolver, LKHSolver, SolvingTrait};
use tspf::{TspBuilder, TspKind};

use crate::logic::solvers::VRPTourWriter;

impl From<&SolveCommand> for Box<dyn SolvingTrait> {
    fn from(options: &SolveCommand) -> Self {
        match options.solver {
            SolverOption::Lkh => Box::new(LKHSolver {
                binary: String::from("./bin/LKH"),
                lkh_solution: options.lkh_solution.clone(),
            }),
            SolverOption::Simulated => Box::new(HybridTspSolver {
                quantum_type: logic::solvers::HybridTspSolverType::Simulated,
                qubo_solution: options.qubo_solution.clone(),
            }),
            SolverOption::LeapHybrid => Box::new(HybridTspSolver {
                quantum_type: logic::solvers::HybridTspSolverType::LeapHybrid,
                qubo_solution: options.qubo_solution.clone(),
            }),
            SolverOption::QbSolv => Box::new(HybridTspSolver {
                quantum_type: logic::solvers::HybridTspSolverType::QbSolv,
                qubo_solution: options.qubo_solution.clone(),
            }),
            SolverOption::Direct => Box::new(HybridTspSolver {
                quantum_type: logic::solvers::HybridTspSolverType::Direct,
                qubo_solution: options.qubo_solution.clone(),
            }),
            SolverOption::SolutionFromFile => Box::new(FileSolver {
                solution_file_dir: options.solution_dir.clone(),
            }),
        }
    }
}

impl From<&OnlySolveCommand> for Box<dyn SolvingTrait> {
    fn from(options: &OnlySolveCommand) -> Self {
        match options.solver {
            SolverOption::Lkh => Box::new(LKHSolver {
                binary: String::from("./bin/LKH"),
                lkh_solution: options.lkh_solution.clone(),
            }),
            SolverOption::Simulated => Box::new(HybridTspSolver {
                quantum_type: logic::solvers::HybridTspSolverType::Simulated,
                qubo_solution: options.qubo_solution.clone(),
            }),
            SolverOption::LeapHybrid => Box::new(HybridTspSolver {
                quantum_type: logic::solvers::HybridTspSolverType::LeapHybrid,
                qubo_solution: options.qubo_solution.clone(),
            }),
            SolverOption::QbSolv => Box::new(HybridTspSolver {
                quantum_type: logic::solvers::HybridTspSolverType::QbSolv,
                qubo_solution: options.qubo_solution.clone(),
            }),
            SolverOption::Direct => Box::new(HybridTspSolver {
                quantum_type: logic::solvers::HybridTspSolverType::Direct,
                qubo_solution: options.qubo_solution.clone(),
            }),
            SolverOption::SolutionFromFile => Box::new(FileSolver {
                solution_file_dir: options.solution_dir.clone(),
            }),
        }
    }
}

fn main() {
    let args = VRPSolverArgs::parse();

    match args.command {
        VRPCommand::Solve(subcommandargs) => {
            let path = subcommandargs.path.clone();
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
                                map_file_path: subcommandargs.cluster_file.clone(),
                            })
                        }
                    },
                    solving_strat: Box::new(VrpSolver {
                        cluster_strat: Box::new(ClusterTspClustering {}),
                        solving_strat: Box::<dyn SolvingTrait>::from(&subcommandargs),
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
                                map_file_path: subcommandargs.cluster_file.clone(),
                            })
                        }
                    },
                    solving_strat: Box::<dyn SolvingTrait>::from(&subcommandargs),
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

                solver.partial_cluster(&cluster_opt.path[..], &vrp);
            }
            args::PartialSolveSubCommand::Solve(solve_opt) => {
                let solver = Box::<dyn SolvingTrait>::from(&solve_opt);

                let path = &solve_opt.path[..];

                let solution = solver.solve(path, Option::Some(solve_opt.transform_only));

                if solve_opt.transform_only {
                    return;
                }

                let vrp = TspBuilder::parse_path(path).unwrap();

                if vrp.kind() != TspKind::Cvrp {
                    panic!(
                        "Invalid TSPLIB instance type {}. (supported is CVRP)",
                        vrp.kind().to_string().to_uppercase()
                    );
                }

                let file_dir = Path::new(path).parent().unwrap().to_str().unwrap();
                let file_name = Path::new(path).file_stem().unwrap().to_str().unwrap();

                let mut file =
                    match std::fs::File::create(format!("{}/{}.sol", file_dir, file_name)) {
                        Ok(file) => file,
                        Err(e) => {
                            println!("Problem opening solution file {e}");
                            exit(1)
                        }
                    };

                println!("writing tours to file {file_dir}/{file_name}.sol");
                (&vrp, &solution).write_tours(&mut file).unwrap();
            }
        },
    }
}
