use petgraph::graph::{Graph, NodeIndex};
use rand::Rng;
use bevy::prelude::*;

use crate::data::{EdgeData, GlobalState, NodeData};

pub fn generate_graph(n_nodes: usize, n_neighbors: usize, distance: f32) -> Graph::<NodeData, EdgeData> {
    let mut graph = Graph::<NodeData, EdgeData>::new();

    let mut current_pos = Vec3::new(0.0, 0.0, 0.0);
    let mut node_data = NodeData::from(current_pos);
    node_data.color = Color::BLUE;

    let mut source = graph.add_node(node_data);

    let mut rng = rand::thread_rng();
    for _ in 0..n_nodes {
        for i in 0..rng.gen_range(1..=n_neighbors) {
            let mut neighbor_pos = current_pos.clone();
            neighbor_pos.x = rng.gen_range(0.0..=distance);
            neighbor_pos.y = rng.gen_range(0.0..=distance / 3.0);
            neighbor_pos.z = rng.gen_range(0.0..=distance);

            let neighbor = graph.add_node(NodeData::from(neighbor_pos));
            graph.add_edge(source, neighbor, EdgeData::default());

            if i == n_neighbors-1 {
                source = neighbor;
                current_pos = neighbor_pos;
            }
        }        
    }

    return graph;
}