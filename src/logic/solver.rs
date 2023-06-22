use std::collections::HashMap;

use kmeans::{KMeans, KMeansConfig};
use tspf::{Point, Tsp, TspSerializer};

use crate::tsplib::serializer::serialize_tsp;

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
    pub fn cluster_tsps(&self, problem: &Tsp, clusters: ClutserOutput) -> Vec<Tsp> {
        let mut tsps: Vec<Tsp> = Vec::new();
        for (i, cluster) in clusters.iter().enumerate() {
            let node_coords = problem
                .node_coords()
                .iter()
                .filter(|(u, point)| cluster.contains(*u) || problem.depots().contains(*u))
                .map(|(u, p)| (*u, p.clone()))
                .collect();

            let node_coords_filter: HashMap<usize, &Point> = problem
                .node_coords()
                .iter()
                .filter(|(u, point)| cluster.contains(*u) || problem.depots().contains(*u))
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
                    .filter(|(u, point)| node_coords_filter.get(u).is_some())
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

    pub fn solve(&self, problem: Tsp) -> u32 {
        let clusters = self.cluster_strat.cluster(&problem);
        let vrps = self.cluster_tsps(&problem, clusters);
        println!("Length: {}", vrps.len());
        for vrp in vrps {
            match TspSerializer::serialize_file(&vrp, String::from(format!("./{}.vrp", vrp.name())))
            {
                Err(err) => {
                    println!("{}", err);
                }
                _ => (),
            };
        }
        1
    }
}
