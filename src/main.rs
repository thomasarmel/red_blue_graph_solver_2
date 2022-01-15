use std::time::Instant;

mod flat_graph;
mod flat_sequence_finder;

fn main() {
    let mut flat_graph = flat_graph::FlatGraph::new(1000);
    flat_graph.generate_random(0.5, 0.5, 0.5);

    //println!("{}", flat_graph);
    let mut flat_sequence_finder =
        flat_sequence_finder::FlatSequenceFinder::new(&flat_graph, flat_graph::Color::RED);
    let start = Instant::now();
    flat_sequence_finder.calculate_sequence_max();
    let duration = start.elapsed();
    /*for node in flat_sequence_finder.sequence_max.iter() {
        println!("{}", node);
    }*/
    println!("{} nanoseconds elapsed", duration.as_nanos());
}
