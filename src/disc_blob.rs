//TODO:
//PUT THIS IN A MODULE WITH OTHER BLOB TYPES

use petgraph::graph::Graph;
use rand_chacha::ChaCha8Rng;
use bevy::prelude::*;

use crate::data::{EdgeData, NodeData, NodeType, Universe};
use crate::node_utils;

pub fn generate_disc_blob(universe: &Universe, mut rng: &mut ChaCha8Rng) 
    -> Graph::<NodeData, EdgeData> {
    let mut graph = Graph::<NodeData, EdgeData>::new();
    
    //TODO: Arguments-to-be
    let radius: f32 = 20.0;
    let height: f32 = 10.0;

    //Create the first blob origin
    
    let origin_pos = node_utils::rand_position(
        universe.size.radius, 
        universe.size.height, 
        Vec3::new(0.0, 0.0, 0.0), 
        &mut rng
    );

    let mut origin_data = NodeData::from(origin_pos);
    origin_data.color = Color::BLUE;
    origin_data.role = NodeType::Center;
    graph.add_node(origin_data);

    for _ in 0..universe.n_nodes-1 {
        //TODO: Check that no other indices are close, then try again

        let member_pos = node_utils::rand_position(radius, height, origin_pos, rng);
        graph.add_node(NodeData::from(member_pos));
    }

    graph = node_utils::calculate_blob_proximity(graph, rng);
    graph = node_utils::connect_members(graph, rng);

    return graph;
}