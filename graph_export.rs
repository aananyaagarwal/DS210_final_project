// graph_export.rs
use crate::network_graph::SocialNetwork;
use petgraph::graph::DiGraph; // Use DiGraph for directed graphs
use petgraph::dot::{Dot, Config};
use std::fs::File;
use std::io::Write;

// This function exports a SocialNetwork structure to a DOT format file using the petgraph library.
pub fn export_to_dot(network: &SocialNetwork) -> std::io::Result<()> {
    let mut graph = DiGraph::<usize, f64>::new();

    // Create a hashmap to track the mapping from node IDs to their corresponding indices in the graph.
    let mut node_indices = std::collections::HashMap::new();

    for (&node, neighbors) in &network.adjacency_list {
        let node_index = *node_indices.entry(node).or_insert_with(|| graph.add_node(node));

        for &neighbor in neighbors {
            let neighbor_index = *node_indices.entry(neighbor).or_insert_with(|| graph.add_node(neighbor));
            
            let weight = 1.0; 
            graph.add_edge(node_index, neighbor_index, weight);
        }
    }

    // Configure the graph visualization without labels on the edges for clarity.
    let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);

    // Attempt to create a file to save the DOT data.
    let mut file = File::create("network.dot")?;
    writeln!(file, "{:?}", dot)?;

    println!("Graph exported to network.dot");

    Ok(())
}
