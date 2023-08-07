use super::common::{ClusterOutput, ClusteringTrait};
use kmeans::{KMeans, KMeansConfig};
use tspf::{Point, Tsp};

pub struct KNNClustering {
    pub count: usize,
}
impl ClusteringTrait for KNNClustering {
    fn cluster(&self, problem: &Tsp) -> ClusterOutput {
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
