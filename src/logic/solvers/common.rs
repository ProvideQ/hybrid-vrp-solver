pub type SolvingOutput = Vec<Vec<usize>>;

pub trait SolvingTrait {
    fn solve(&self, path: &str) -> SolvingOutput;
}
