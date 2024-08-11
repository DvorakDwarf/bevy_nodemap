//TODO:
//PUT THIS IN A MODULE WITH OTHER BLOB TYPES

use petgraph::graph::{Graph, UnGraph};
use petgraph::Undirected;
use rand_chacha::ChaCha8Rng;
use bevy::prelude::*;

use crate::blob_utils::is_blob_clipping;
use crate::data::{EdgeData, NodeData, NodeType, Universe};
use crate::node_utils::{self, is_member_clipping};

pub fn generate_disc_blob(
    universe: &Universe, 
    center_postions: &Vec<Vec3>,
    mut rng: &mut ChaCha8Rng) -> UnGraph<NodeData, EdgeData>
{
    let mut graph = UnGraph::<NodeData, EdgeData>::new_undirected();
    
    //Maybe these should be arguments ?
    let radius: f32 = 20.0;
    let height: f32 = 7.0;

    //Create the first blob origin
    let mut origin_pos = Vec3::ZERO;
    loop {
        origin_pos = node_utils::rand_position(
            universe.size.radius, 
            universe.size.height, 
            Vec3::new(0.0, 0.0, 0.0), 
            &mut rng
        );

        let blob_clipping = is_blob_clipping(
            center_postions, 
            origin_pos, 
            universe.blob_distance_tolerance
        );
        if blob_clipping == false {
            break;
        }
    }

    let mut origin_data = NodeData::from(origin_pos);
    origin_data.color = Color::BLUE;
    origin_data.role = NodeType::Center;
    graph.add_node(origin_data);

    for _ in 0..universe.n_nodes-1 {
        //Check that no other indices are close, then try again
        loop {
            let member_pos = node_utils::rand_position(radius, height, origin_pos, rng);
            if is_member_clipping(&graph, &member_pos, universe.no_no_distance) == false {
                graph.add_node(NodeData::from(member_pos));
                break;
            }   
        }

    }

    graph = node_utils::calculate_blob_proximity(graph, rng);
    // graph = node_utils::connect_members(graph, rng, universe.n_member_candidates);

    //TODO: JUST FOR TESTING
    match center_postions.len() {
        // 1 => {graph = node_utils::connect_members_no_shuffle(
        //         graph, 
        //         rng, 
        //         universe.n_member_candidates)},
        _ => graph = node_utils::connect_members(
                graph, 
                rng, 
                universe.n_member_candidates,
                universe.fluff_requirement)
    }

    return graph;
}