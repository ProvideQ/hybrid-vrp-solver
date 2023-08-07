use super::common::{ClusterOutput, ClusteringTrait};
use kmeans::{KMeans, KMeansConfig};
use tspf::{Point, Tsp};

pub struct ClusterTspClustering {}

fn distance(a: &Vec<f64>, b: &Vec<f64>) -> Option<f64> {
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

impl ClusterTspClustering {
    fn choose_core<'a>(&'a self, problem: &Tsp, rest: &'a Vec<usize>) -> Option<&usize> {
        rest.first()
    }

    fn center(&self, problem: &Tsp, cluster: &Vec<usize>) -> Vec<f64> {
        if cluster.len() == 0 {
            return vec![
                0.0f64;
                problem
                    .node_coords()
                    .values()
                    .collect::<Vec<&Point>>()
                    .first()
                    .unwrap()
                    .pos()
                    .len()
            ];
        }
        problem
            .node_coords()
            .values()
            .filter(|p| cluster.contains(&p.id()))
            .map(|p| p.pos().clone())
            .reduce(|s, p| {
                s.iter()
                    .zip(p.iter())
                    .map(|v| v.0 + v.1)
                    .collect::<Vec<f64>>()
            })
            .unwrap()
            .iter()
            .map(|c| c / (cluster.len() as f64))
            .collect()
    }

    fn cluster_demand(&self, problem: &Tsp, cluster: &Vec<usize>) -> f64 {
        problem
            .demands()
            .iter()
            .filter_map(|(id, demand)| {
                if cluster.contains(id) {
                    Some(demand)
                } else {
                    None
                }
            })
            .sum()
    }
}

impl ClusteringTrait for ClusterTspClustering {
    fn cluster(&self, problem: &Tsp) -> ClusterOutput {
        let mut not_assigned: Vec<usize> = problem
            .node_coords()
            .keys()
            .filter_map(|id| {
                if problem.depots().contains(id) {
                    return None;
                }
                Some(*id)
            })
            .collect();

        let mut assignments: Vec<Vec<usize>> = vec![];

        loop {
            let core = match self.choose_core(problem, &not_assigned) {
                Some(core) => core,
                None => break,
            };

            let mut cluster = vec![];

            'inner: loop {
                let center = self.center(problem, &cluster);

                let mut not_assigned_points: Vec<&Point> = problem
                    .node_coords()
                    .values()
                    .filter(|p| not_assigned.contains(&p.id()))
                    .collect();

                not_assigned_points.sort_by(|a, b| {
                    distance(a.pos(), &center)
                        .unwrap()
                        .total_cmp(&distance(b.pos(), &center).unwrap())
                });

                let closest = match not_assigned_points.first() {
                    Some(point) => point,
                    None => break 'inner,
                };

                if (problem.demands().get(&closest.id()).unwrap()
                    + self.cluster_demand(problem, &cluster)
                    <= problem.capacity())
                {
                    cluster.push(closest.id());
                    not_assigned = not_assigned
                        .iter()
                        .filter(|id| **id != closest.id())
                        .map(|id| *id)
                        .collect()
                } else {
                    break 'inner;
                }
            }

            println!("{:?}", cluster);
            assignments.push(cluster);
        }
        assignments
    }
}
