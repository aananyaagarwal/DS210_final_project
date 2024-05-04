// Import necessary standard library collections and I/O capabilities
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

// Define a structure to represent a social network graph with an adjacency list
#[derive(Debug)]
pub struct SocialNetwork {
    pub adjacency_list: HashMap<usize, Vec<usize>>,
}

impl SocialNetwork {
    // Constructor for creating a new, empty SocialNetwork
    pub fn new() -> Self {
        SocialNetwork {
            adjacency_list: HashMap::new(),
        }
    }

    // Adds a node to the network if it does not already exist
    pub fn add_node(&mut self, node: usize) {
        self.adjacency_list.entry(node).or_insert_with(Vec::new);
    }

    // Creates a directed edge from one node to another
    pub fn connect_nodes(&mut self, from: usize, to: usize) {
        self.adjacency_list.entry(from).or_insert_with(Vec::new).push(to);
    }
    
    // Computes the shortest path lengths from a starting node to all other reachable nodes using BFS
    pub fn bfs_path_lengths(&self, start: usize) -> HashMap<usize, usize> {
        let mut distances = HashMap::new();
        let mut to_visit = VecDeque::new();
        let mut seen = HashSet::new();

        // Initialize the queue with the start node and mark it as seen
        to_visit.push_back(start);
        distances.insert(start, 0);
        seen.insert(start);

        // Continue until there are no more nodes to visit
        while let Some(current) = to_visit.pop_front() {
            let current_distance = *distances.get(&current).unwrap();

            // Explore each neighbor of the current node
            if let Some(neighbors) = self.adjacency_list.get(&current) {
                for &neighbor in neighbors {
                    // If the neighbor hasn't been seen, add it to the queue and update distances
                    if seen.insert(neighbor) {
                        distances.insert(neighbor, current_distance + 1);
                        to_visit.push_back(neighbor);
                    }
                }
            }
        }

        distances
    }
    // Loads a social network from a text file where each line contains two integers representing an edge
    pub fn load_from(file_path: &str) -> std::io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut network = SocialNetwork::new();
        for line in reader.lines() {
            let line = line?;
            let nodes: Vec<&str> = line.split_whitespace().collect();
            if nodes.len() == 2 {
                let source = nodes[0].parse::<usize>().unwrap();
                let destination = nodes[1].parse::<usize>().unwrap();
                // Ensures both source and destination nodes are added to the network
                network.add_node(source);
                network.add_node(destination);
                // Connects the source to the destination
                network.connect_nodes(source, destination);
            }
        }
        Ok(network)
    }
}
