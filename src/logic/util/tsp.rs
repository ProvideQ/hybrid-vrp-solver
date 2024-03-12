use tspf::Tsp;

pub trait Distancing<T> {
    fn distance(&self, a: usize, b: usize) -> Option<T>;
}

impl Distancing<f64> for Tsp {
    fn distance(&self, a: usize, b: usize) -> Option<f64> {
        let a = match self.node_coords().get(&a) {
            Some(p) => p.pos(),
            None => {
                return None;
            }
        };
        let b = match self.node_coords().get(&b) {
            Some(p) => p.pos(),
            None => {
                return None;
            }
        };
        if a.len() != b.len() {
            return None;
        }
        Some(
            a.iter()
                .zip(b.iter())
                .map(|(av, bv)| (av - bv) * (av - bv))
                .reduce(|a, b| a + b)
                .unwrap()
                .sqrt(),
        )
    }
}

pub fn calculate_solution_score(problem: &Tsp, paths: &Vec<Vec<usize>>) -> f64 {
    paths
        .iter()
        .map(|path| {
            let mut length: f64 = 0f64;
            let mut iter = path.iter();
            let first = iter.next().unwrap();

            let mut last = first;
            for next in iter {
                length += problem.distance(*last, *next).unwrap();
                last = next;
            }
            length += problem.distance(*last, *first).unwrap();
            length
        })
        .sum()
}
