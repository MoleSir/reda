use reda_partition::{algorithm::{BisectionGraphPartition, KernighanLin}, graph::Graph};

fn main_result() -> Result<(), Box<dyn std::error::Error>> {
    let graph = Graph::<i32>::load_file("./data/graph.txt")?;
    let algo = KernighanLin;
    algo.partition(&graph);

    Ok(())
}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("Error: {}", e)
    }
}