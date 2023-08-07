use tspf::Tsp;

pub type ClusterOutput = Vec<Vec<usize>>;

pub trait ClusteringTrait {
    fn cluster(&self, problem: &Tsp) -> ClusterOutput;
}
