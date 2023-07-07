use tspf::Tsp;

pub type ClutserOutput = Vec<Vec<usize>>;

pub trait ClusteringTrait {
    fn cluster(&self, problem: &Tsp) -> ClutserOutput;
}
