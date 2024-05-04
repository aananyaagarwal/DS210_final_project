use std::collections::{HashMap, HashSet};
use crate::network_graph::SocialNetwork;

// Calculate Jaccard similarity scores between all pairs of nodes that are exactly two edges apart.
pub fn calculate_jaccard_scores(graph: &SocialNetwork) -> HashMap<(usize, usize), f64> {
    let mut results = HashMap::new(); 
    let nodes = graph.adjacency_list.keys().copied().collect::<Vec<usize>>(); 

    for &node1 in &nodes {
        let paths = graph.bfs_path_lengths(node1); 

        // Iterate over each node again to calculate pairwise Jaccard similarity
        for &node2 in &nodes {
            if let Some(&distance) = paths.get(&node2) {
               
                if distance == 2 {
                    
                    let set1: HashSet<usize> = graph.adjacency_list.get(&node1).unwrap_or(&vec![]).iter().copied().collect();
                    let set2: HashSet<usize> = graph.adjacency_list.get(&node2).unwrap_or(&vec![]).iter().copied().collect();

                    
                    let intersection = set1.intersection(&set2).count();
                    let union = set1.union(&set2).count();
                    let score = intersection as f64 / union as f64;

                    results.insert((node1, node2), score);
                }
            }
        }
    }

    results
}

// Summarize the calculated Jaccard scores into average, maximum, and list of most similar pairs.
pub fn summarize_scores(scores: &HashMap<(usize, usize), f64>) -> (f64, f64, Vec<((usize, usize), f64)>) {
    let total: f64 = scores.values().sum(); 
    let count = scores.len() as f64; 
    let avg = total / count; 

    let mut max_score = 0.0; 
    let mut max_pairs = vec![]; 

    // Determine maximum score and corresponding pairs
    for (&pair, &score) in scores.iter() {
        if score > max_score {
            max_score = score;
            max_pairs = vec![(pair, score)]; 
        } else if score == max_score {
            max_pairs.push((pair, score)); 
        }
    }

    (avg, max_score, max_pairs)
}

// Compute the percentage of node pairs exceeding specific Jaccard similarity thresholds.
pub fn compute_thresholds(scores: &HashMap<(usize, usize), f64>) -> Vec<(f64, f64)> {
    let thresholds = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0]; 
    thresholds.iter().map(|&threshold| {
        let count = scores.values().filter(|&&score| score > threshold).count(); 
        let percentage = count as f64 / scores.len() as f64 * 100.0; 
        (threshold, percentage) 
    }).collect()
}
