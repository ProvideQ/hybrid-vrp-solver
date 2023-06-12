use std::collections::LinkedList;

use tspf::Tsp;

pub trait ClusteringTrait {
    fn cluster(&self, problem: Tsp) -> LinkedList<LinkedList<u32>>;
}

pub struct KNNClustering;
impl ClusteringTrait for KNNClustering {
    fn cluster(&self, _problem: Tsp) -> LinkedList<LinkedList<u32>> {
        LinkedList::new()
    }
}

pub struct VrpSolver {
    pub cluster_strat: Box<dyn ClusteringTrait>,
}
impl VrpSolver {
    pub fn solve(&self, problem: Tsp) -> u32 {
        self.cluster_strat.cluster(problem);
        1
    }
}
