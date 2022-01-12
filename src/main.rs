mod flat_graph;

fn main() {
    let mut flat_graph = flat_graph::FlatGraph::new(10);
    flat_graph.generate_random(0.5, 0.5, 0.5);
    //println!("{}", flat_graph);
    println!("Hello, world!");
}
