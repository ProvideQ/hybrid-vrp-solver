use std::fs::File;
use std::io::Write;
use std::{format, process::exit};
use tspf::{CoordKind, Tsp, TspKind};

pub fn serialize_tsp(tsp: &Tsp, path: String) {
    let output_file = match File::create(path) {
        Ok(file) => file,
        _ => exit(1),
    };
    let mut writer = std::io::BufWriter::new(output_file);

    writeln!(writer, "NAME: {}", tsp.name());
    writeln!(writer, "TYPE: {}", tsp.kind().to_string().to_uppercase());
    writeln!(writer, "COMMENT: {}", tsp.comment());
    writeln!(writer, "DIMENSION: {}", tsp.dim());
    writeln!(writer, "CAPACITY: {}", tsp.capacity());
    writeln!(
        writer,
        "EDGE_WEIGHT_TYPE: {}",
        camel_to_snake(tsp.weight_kind().to_string()).to_uppercase()
    );
    writeln!(
        writer,
        "EDGE_WEIGHT_FORMAT: {}",
        camel_to_snake(tsp.weight_format().to_string()).to_uppercase()
    );
    writeln!(
        writer,
        "EDGE_DATA_FORMAT: {}",
        camel_to_snake(tsp.edge_format().to_string()).to_uppercase()
    );
    writeln!(
        writer,
        "NODE_COORD_TYPE: {}",
        camel_to_snake(tsp.coord_kind().to_string()).to_uppercase()
    );
    writeln!(
        writer,
        "DISP_DATA_TYPE: {}",
        camel_to_snake(tsp.disp_kind().to_string()).to_uppercase()
    );

    if tsp.coord_kind() != CoordKind::NoCoord {
        let data = match tsp
            .node_coords()
            .iter()
            .map(|x| {
                format!(
                    "{} {}\n",
                    x.0,
                    match x
                        .1
                        .pos()
                        .iter()
                        .map(|c| c.to_string())
                        .reduce(|t, c| format!("{} {}", t, c))
                    {
                        Some(d) => d,
                        _ => exit(1),
                    }
                )
            })
            .reduce(|str, poi| format!("{}\n{}", str, poi))
        {
            Some(data) => data,
            _ => exit(1),
        };
        writeln!(writer, "NODE_COORD_SECTION: \n{}", data);
    }

    if tsp.kind() == TspKind::Cvrp {
        writeln!(
            writer,
            "DEPOT_SECTION: \n{}\n-1",
            match tsp
                .depots()
                .iter()
                .map(|i| i.to_string())
                .reduce(|str, i| format!("{}\n{}", str, i))
            {
                Some(data) => data,
                _ => exit(1),
            }
        );
    }

    if tsp.kind() == TspKind::Cvrp {
        writeln!(
            writer,
            "DEMAND_SECTION: \n{}",
            match tsp
                .demands()
                .iter()
                .map(|(i, d)| format!("{} {}", i, d))
                .reduce(|str, id| format!("{}\n{}", str, id))
            {
                Some(data) => data,
                _ => exit(1),
            }
        );
    }
    writeln!(writer, "EOF");
}

fn camel_to_snake(str: String) -> String {
    str.chars()
        .flat_map(|c| {
            if c.is_uppercase() {
                vec!['_', c.to_lowercase().next().unwrap()]
            } else {
                vec![c]
            }
        })
        .collect::<String>()
}
