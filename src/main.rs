use std::time::Instant;
use red_blue_graph::{flat_graph, flat_sequence_finder};

fn main() {
    let mut flat_graph = flat_graph::FlatGraph::new(10);
    flat_graph.generate_random(0.5, 0.5, 0.5);
    println!("Test on a graph of size {}", flat_graph.max_capacity);

    println!("{}", flat_graph);
    let flat_sequence_finder =
        flat_sequence_finder::FlatSequenceFinder::new(&flat_graph, flat_graph::Color::RED);
    let start = Instant::now();
    let sequence_max = flat_sequence_finder.calculate_sequence_max();
    let duration = start.elapsed();
    for node in sequence_max.iter() {
        println!("{}", node);
    }
    println!("{}", flat_graph);
    println!("{} nanoseconds elapsed", duration.as_nanos());
}
