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
    pub fn cluster_tsps(&self, problem: Tsp, clusters: ClutserOutput) -> Vec<Tsp> {}

    pub fn solve(&self, problem: Tsp) -> u32 {
        let clusters = self.cluster_strat.cluster(problem);
        let vrps = self.cluster_tsps(problem, clusters);
        1
    }
}
