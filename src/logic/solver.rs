use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader},
    process::exit,
    sync::Arc,
};

use kmeans::{KMeans, KMeansConfig};
use tspf::{Point, Tsp, TspSerializer};
use vrp_scientific::{
    core::{
        rosomaxa::prelude::TelemetryMode,
        solver::{create_default_config_builder, Solver},
        utils::Environment,
    },
    tsplib::*,
};

pub type ClutserOutput = Vec<Vec<usize>>;

pub trait ClusteringTrait {
    fn cluster(&self, problem: &Tsp) -> ClutserOutput;
}

pub struct KNNClustering {
    pub count: usize,
}
impl ClusteringTrait for KNNClustering {
    fn cluster(&self, problem: &Tsp) -> ClutserOutput {
        let points = problem
            .node_coords()
            .iter()
            .map(|(_u, p)| p)
            .filter(|p| !problem.depots().contains(&p.id()))
            .collect::<Vec<&Point>>();

        let coords = points
            .iter()
            .map(|p| p.pos())
            .flatten()
            .map(|v| *v)
            .collect::<Vec<f64>>();

        let kmean = KMeans::new(coords, problem.dim() - problem.depots().len(), 2);

        let result = kmean.kmeans_lloyd(
            self.count,
            100,
            KMeans::init_kmeanplusplus,
            &KMeansConfig::default(),
        );

        result
            .assignments
            .iter()
            .enumerate()
            .fold(vec![vec![]; self.count], |mut x, (i, y)| {
                x[*y].push(points[i].id());
                x
            })
    }
}

pub struct VrpSolver {
    pub cluster_strat: Box<dyn ClusteringTrait>,
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

    fn partial_solve(&self, vrp_file: &File) -> Vec<Vec<usize>> {
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
                    exit(1);
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

    pub fn solve(&self, problem: Tsp) -> Vec<Vec<usize>> {
        let clusters = self.cluster_strat.cluster(&problem);
        let vrps = self.cluster_tsps(&problem, clusters);
        println!("Length: {}", vrps.len());
        let mut vrp_files: Vec<File> = vec![];

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
            .map(|file| self.partial_solve(file))
            .reduce(|paths, new_paths| [paths, new_paths].concat())
            .unwrap_or_else(|| {
                println!("Something went wrong reducing paths");
                vec![]
            });

        paths
    }
}
