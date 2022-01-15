mod flat_graph;
mod flat_sequence_finder;

fn main() {
    let mut flat_graph = flat_graph::FlatGraph::new(10);
    flat_graph.generate_random(0.5, 0.5, 0.5);

    println!("{}", flat_graph);
    let mut flat_sequence_finder =
        flat_sequence_finder::FlatSequenceFinder::new(&flat_graph, flat_graph::Color::RED);
    flat_sequence_finder.get_sequence_max();
    for node in flat_sequence_finder.sequence_max.iter() {
        println!("{}", node);
    }
}
