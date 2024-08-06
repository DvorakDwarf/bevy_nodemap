//TODO:
//PUT THIS IN A MODULE WITH OTHER BLOB TYPES

use petgraph::graph::Graph;
use rand_chacha::ChaCha8Rng;
use bevy::prelude::*;

use crate::data::{EdgeData, NodeData, NodeType, Universe};
use crate::node_utils::{self, is_member_clipping};

//TODO: Move to blob_utils
fn is_blob_clipping(
    center_postions: &Vec<Vec3>, 
    origin_pos: Vec3) -> bool 
{
    //TODO: Arguments-to-be
    let distance_tolerance = 30.0;

    for position in center_postions {
        if origin_pos.distance(*position) < distance_tolerance {
            return true;
        }
    }

    return false;
}

pub fn generate_disc_blob(
    universe: &Universe, 
    center_postions: &Vec<Vec3>,
    mut rng: &mut ChaCha8Rng) -> Graph::<NodeData, EdgeData> 
{
    let mut graph = Graph::<NodeData, EdgeData>::new();
    
    //TODO: Arguments-to-be
    let radius: f32 = 20.0;
    let height: f32 = 10.0;

    //Create the first blob origin
    let mut origin_pos = Vec3::ZERO;
    loop {
        origin_pos = node_utils::rand_position(
            universe.size.radius, 
            universe.size.height, 
            Vec3::new(0.0, 0.0, 0.0), 
            &mut rng
        );

        if is_blob_clipping(center_postions, origin_pos) == false {
            break;
        }
    }

    let mut origin_data = NodeData::from(origin_pos);
    origin_data.color = Color::BLUE;
    origin_data.role = NodeType::Center;
    graph.add_node(origin_data);

    for _ in 0..universe.n_nodes-1 {
        //TODO: Check that no other indices are close, then try again
        loop {
            let member_pos = node_utils::rand_position(radius, height, origin_pos, rng);
            if is_member_clipping(&graph, &member_pos) == false {
                graph.add_node(NodeData::from(member_pos));
                break;
            }   
        }

    }

    graph = node_utils::calculate_blob_proximity(graph, rng);
    graph = node_utils::connect_members(graph, rng);

    return graph;
}