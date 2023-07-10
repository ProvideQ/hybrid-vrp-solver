use super::{DummySolver, SolvingTrait};

pub struct LKHSolver {
    pub binary: String,
}

impl SolvingTrait for LKHSolver {
    fn solve(&self, path: &str) -> super::SolvingOutput {
        let solver = DummySolver {};
        solver.solve(path)
    }
}
