use super::common::{ClusterOutput, ClusteringTrait};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct FileClustering {
    pub map_file_path: String,
}

impl ClusteringTrait for FileClustering {
    fn cluster(&self, _problem: &tspf::Tsp) -> ClusterOutput {
        let file = File::open(&self.map_file_path).unwrap();
        let reader = BufReader::new(file);
        let mut clusters = vec![];
        let mut cluster: Vec<(usize, usize)> = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            if line == "-1" {
                cluster.sort_by_key(|(_, i)| *i);
                clusters.push(cluster.iter().map(|(id, _)| *id).collect());
                cluster = vec![];
                continue;
            }
            let mapping = line
                .splitn(2, ' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            cluster.push((mapping[0], mapping[1]));
        }
        clusters
    }
}
