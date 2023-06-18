use std::{collections::LinkedList, process::exit};

use kmeans::{KMeans, KMeansConfig};
use tspf::{Point, Tsp, TspBuilder};

pub type ClutserOutput = Vec<Vec<usize>>;

pub trait ClusteringTrait {
    fn cluster(&self, problem: Tsp) -> ClutserOutput;
}

pub struct KNNClustering {
    pub count: usize,
}
impl ClusteringTrait for KNNClustering {
    fn cluster(&self, problem: Tsp) -> ClutserOutput {
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
    pub fn cluster_tsps(&self, problem: Tsp, clusters: ClutserOutput) -> Vec<Tsp> {
        let mut tsps: Vec<Tsp> = Vec::new();
        for (i, cluster) in clusters.iter().enumerate() {
            let node_coords = problem
                .node_coords()
                .iter()
                .filter(|(u, point)| cluster.contains(*u) || problem.depots().contains(*u))
                .collect();
            let tsp = Tsp {
                name: problem.name() + "_" + i.to_string(),
                kind: tspf::TspKind::Cvrp,
                comment: problem.comment() + " - Cluster Nr." + i.to_string(),
                dim: problem.depots().len() + cluster.len(),
                capacity: problem.capacity(),
                weight_kind: problem.weight_kind(),
                weight_format: problem.weight_format(),
                edge_format: problem.edge_format(),
                coord_kind: problem.coord_kind(),
                disp_kind: problem.disp_kind(),
                node_coords,
                depots: problem.depots(),
                demands: problem
                    .demands()
                    .iter()
                    .filter(|(u, point)| node_coords.get(u).is_some())
                    .collect(),
                fixed_edges: problem
                    .fixed_edges()
                    .iter()
                    .filter(|edge| {
                        node_coords.get(&edge.0).is_some() && node_coords.get(&edge.1).is_some()
                    })
                    .collect(),
                disp_coords: problem.disp_coords(),
                edge_weights: problem.edge_weights(),
                tours: vec![],
            };
            tsps.push(tsp);
        }
        tsps
    }

    pub fn solve(&self, problem: Tsp) -> u32 {
        let clusters = self.cluster_strat.cluster(problem);
        let vrps = self.cluster_tsps(problem, clusters);
        1
    }
}
