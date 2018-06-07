use std::hash::Hash;
use std::fmt;
use std::ops::Add;

extern crate graph;
use graph::graph::Edge;
use graph::algorithm::kruskal;

fn read_graph<T>(filename : &str) -> Option<Vec<Edge<T>>>
    where T : PartialEq + Hash + Ord + Clone + fmt::Debug + Add + std::str::FromStr
{
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    let path = Path::new(filename);

    let mut file = match File::open(&path) {
        Err(why) => {
            println!("Couldn't open {}: {}", filename, Error::description(&why));
            return None;
        },
        Ok(file) => file,
    };

    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        println!("Couldn't read {}: {}", filename, Error::description(&why));
        return None;
    }

    let edges : Vec<Option<Edge<T>>> = s.replace(" ", "").split("\n")
        .map(|x| {
            let x = x.split(",").collect::<Vec<_>>();
            if let (Some(n1), Some(n2), Some(w)) = (x.get(0), x.get(1), x.get(2)) {
                match w.trim().parse::<T>() {
                    Ok(num) => Some(Edge::new((n1, n2), num)),
                    Err(_) => None,
                }
            } else {
                None
            }}).collect();
    
    if edges.iter().all(|e| e.is_some()) {
        Some(edges.into_iter().filter_map(|e|e).collect::<Vec<_>>())
    } else {
        println!("The file format is not correct");
        None
    }
}

fn main() {
    if let Some(edges) = read_graph::<usize>("edges.csv") {
        let edges = kruskal::run(edges);

        println!("Total Weight: {}", edges.iter().fold(0, |s, e| s + e.weight));

        for e in edges.iter() {
            println!("{:?}", e);
        }
    }
}