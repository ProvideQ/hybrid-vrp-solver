pub type SolvingOutput = Vec<Vec<usize>>;

pub trait SolvingTrait {
    fn solve(&self, path: &str, transform_only: Option<bool>) -> SolvingOutput;
}
