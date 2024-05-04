mod similarity_analysis;
mod network_graph;
mod graph_export;

use similarity_analysis::{calculate_jaccard_scores, summarize_scores, compute_thresholds};
use crate::network_graph::SocialNetwork;
use crate::graph_export::export_to_dot;

fn main() {
    // Attempt to load the network from a file and handle errors by panicking with a clear message
    let network = SocialNetwork::load_from("facebook_combined.txt").unwrap_or_else(|err| {
        panic!("Failed to load graph: {}", err);
    });

    // Calculate and store Jaccard similarity scores for the entire network
    let score_map = calculate_jaccard_scores(&network);
    println!("Computed Jaccard Scores for Vertex Pairs:");
    // Display each pair's Jaccard similarity score
    for ((start, end), score) in &score_map {
        println!("Score between {} and {}: {:.4}", start, end, score);
    }

    // Calculate and display statistical measures of similarity scores
    let (avg_score, highest_score, similar_pairs) = summarize_scores(&score_map);
    println!("Average Jaccard score: {:.3}", avg_score);
    println!("Highest Jaccard score: {:.3}", highest_score);

    // Display pairs with the highest similarity scores
    println!("Most Similar Vertex Pairs:");
    for ((node1, node2), sim_score) in similar_pairs {
        println!("Pair: ({}, {}), Score: {:.3}", node1, node2, sim_score);
    }

    // Calculate and display the percentage of vertex pairs with Jaccard scores above specific thresholds
    let score_percentages = compute_thresholds(&score_map);
    for (threshold, percentage) in score_percentages {
        println!("Percentage of vertex pairs with a Jaccard score above {:.1}: {}%", threshold, percentage);
    }

    // Attempt to export the network graph to a DOT file for visualization and handle any errors
    if let Err(err) = export_to_dot(&network) {
        eprintln!("Failed to export graph: {}", err);
    }
}

// Testing functionalities by drawing a sample dataset and performing calculations by hand 

#[test]
// Test to ensure the graph is loaded correctly and has the expected number of nodes
fn test_graph_loading() {
    let network = SocialNetwork::load_from("test.txt").unwrap();
    assert_eq!(network.adjacency_list.len(), 10, "Loaded graph should have 10 nodes.");
}

#[test]
// Test to verify the Jaccard calculation functionality by checking a specific pair
fn test_jaccard_calculation() {
    let network = SocialNetwork::load_from("test.txt").unwrap();
    let scores = calculate_jaccard_scores(&network);
    assert!(scores.contains_key(&(4, 6)), "Expected to find similarity score for nodes 4 and 6.");
    assert_eq!(scores[&(4, 6)], 0.500);
}

#[test]
// Test to verify that statistical measures of Jaccard similarities are correctly calculated
fn test_find_mean_max_similarity() {
    let network = SocialNetwork::load_from("test.txt").unwrap();
    let scores = calculate_jaccard_scores(&network);
    let (avg_score, highest_score, similar_pairs) = summarize_scores(&scores);
    assert_eq!(avg_score, 0.09523809523809523, "Expected mean similarity is 0.095.");
    assert_eq!(highest_score, 0.500, "Expected maximum similarity is 0.500.");
}

#[test]
// Test to ensure that the breadth-first search (BFS) path functionality is working as expected
fn test_bfs_functionality() {
    let network = SocialNetwork::load_from("test.txt").unwrap();
    let paths = network.bfs_path_lengths(2);
    assert_eq!(paths.get(&8), Some(&2), "Expected shortest path from node 2 to 8 is 2.");
}
