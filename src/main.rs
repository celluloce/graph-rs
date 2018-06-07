extern crate graph;
use graph::graph::Edge;
use graph::algorithm::kruskal;

fn main() {
    let edges = vec![
        Edge::new(("A", "B"), 4),
        Edge::new(("A", "C"), 6),
        Edge::new(("A", "D"), 12),
        Edge::new(("B", "E"), 2),
        Edge::new(("C", "D"), 8),
        Edge::new(("C", "E"), 9),
        Edge::new(("D", "E"), 3),
        Edge::new(("E", "F"), 10),
        Edge::new(("E", "G"), 5),
        Edge::new(("F", "G"), 3),
    ].into_iter().collect();

    let edges = kruskal::run(edges);

    println!("Total Weight: {}", edges.iter().fold(0, |s, e| s + e.weight));

    for e in edges.iter() {
        println!("{:?}", e);
    }

    /*
    Total Weight: 23
    Edge(("A", "C"), 6)
    Edge(("B", "E"), 2)
    Edge(("F", "G"), 3)
    Edge(("D", "E"), 3)
    Edge(("A", "B"), 4)
    Edge(("E", "G"), 5)
    */
}