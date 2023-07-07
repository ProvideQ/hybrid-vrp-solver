use super::{
    super::error_code::ExitCode,
    clustering::{ClusteringTrait, ClutserOutput},
    solvers::{SolvingOutput, SolvingTrait},
};

use std::{
    collections::HashMap,
    fs::{self, File},
    process::exit,
};
use tspf::{Point, Tsp, TspBuilder, TspKind, TspSerializer};

pub struct VrpSolver {
    pub cluster_strat: Box<dyn ClusteringTrait>,
    pub solving_strat: Box<dyn SolvingTrait>,
}
impl VrpSolver {
    fn cluster_tsps(&self, problem: &Tsp, clusters: ClutserOutput) -> Vec<Tsp> {
        let mut tsps: Vec<Tsp> = Vec::new();
        for (i, cluster) in clusters.iter().enumerate() {
            let node_coords = problem
                .node_coords()
                .iter()
                .filter(|(u, _point)| cluster.contains(*u) || problem.depots().contains(*u))
                .map(|(u, p)| (*u, p.clone()))
                .collect();

            let node_coords_filter: HashMap<usize, &Point> = problem
                .node_coords()
                .iter()
                .filter(|(u, _point)| cluster.contains(*u) || problem.depots().contains(*u))
                .map(|(u, p)| (*u, p))
                .collect();

            let tsp = Tsp::from(
                format!("{}_{}", problem.name(), i.to_string()),
                tspf::TspKind::Cvrp,
                format!("{} - Cluster Nr.{}", problem.comment(), i.to_string()),
                problem.depots().len() + cluster.len(),
                problem.capacity(),
                problem.weight_kind(),
                problem.weight_format(),
                problem.edge_format().clone(),
                problem.coord_kind(),
                problem.disp_kind(),
                node_coords,
                problem.depots().iter().filter_map(|d| Some(*d)).collect(),
                problem
                    .demands()
                    .iter()
                    .filter(|(u, _p)| node_coords_filter.get(u).is_some())
                    .map(|(u, p)| (*u, *p))
                    .collect(),
                problem
                    .fixed_edges()
                    .iter()
                    .filter(|edge| {
                        node_coords_filter.get(&edge.0).is_some()
                            && node_coords_filter.get(&edge.1).is_some()
                    })
                    .map(|t| *t)
                    .collect(),
                problem
                    .disp_coords()
                    .iter()
                    .filter_map(|p| match node_coords_filter.get(&p.id()) {
                        Some(_) => Some(p.clone()),
                        None => None,
                    })
                    .collect(),
                problem
                    .edge_weights()
                    .iter()
                    .enumerate()
                    .filter_map(|(i, vec)| match node_coords_filter.get(&i) {
                        Some(_i) => Some(
                            vec.iter()
                                .enumerate()
                                .filter_map(|(j, poi)| match node_coords_filter.get(&j) {
                                    Some(_) => Some(*poi),
                                    None => None,
                                })
                                .collect::<Vec<f64>>(),
                        ),
                        None => None,
                    })
                    .collect(),
                vec![],
            );
            tsps.push(tsp);
        }
        tsps
    }
}

impl SolvingTrait for VrpSolver {
    fn solve(&self, path: &str) -> SolvingOutput {
        let problem = match TspBuilder::parse_path(path) {
            Ok(instance) => instance,
            Err(e) => {
                println!("Problems reading the VRP-Instance: {}", e);
                exit(ExitCode::ReadProblems as i32);
            }
        };
        if problem.kind() != TspKind::Cvrp {
            println!(
                "Invalid TSPLIB instance type {}. (supported is CVRP)",
                problem.kind().to_string().to_uppercase()
            );
            exit(ExitCode::WrongTspType as i32);
        }

        println!("name: {}", problem.name());
        println!("type: {}", problem.kind());

        let clusters = self.cluster_strat.cluster(&problem);
        let vrps = self.cluster_tsps(&problem, clusters);

        let mut vrp_files: Vec<(File, String)> = vec![];

        match fs::create_dir_all("./.vrp") {
            Ok(_) => {}
            Err(e) => {
                println!("Something went wrong creating dirs: \n{e}");
                exit(1)
            }
        };
        for vrp in vrps {
            vrp_files.push(
                match TspSerializer::serialize_file(
                    &vrp,
                    String::from(format!("./.vrp/{}.vrp", vrp.name())),
                ) {
                    Err(err) => {
                        println!("{}", err);
                        exit(1)
                    }
                    Ok(file) => file,
                },
            );
        }

        let paths = vrp_files
            .iter()
            .map(|(file, path)| self.solving_strat.solve(&path[..]))
            .reduce(|paths, new_paths| [paths, new_paths].concat())
            .unwrap_or_else(|| {
                println!("Something went wrong reducing paths");
                vec![]
            });

        paths
    }
}
