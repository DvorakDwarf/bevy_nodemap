//TODO:
//PUT THIS IN A MODULE WITH OTHER BLOB TYPES

use petgraph::graph::UnGraph;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rand::prelude::SliceRandom;
use bevy::prelude::*;

use crate::blob_utils::{extend_blob, is_blob_clipping};
use crate::data::{BlobType, EdgeData, NodeData, NodeType, Universe};
use crate::node_utils::{self, is_member_clipping};

fn get_start_pos(    
    local_graph: &UnGraph<NodeData, EdgeData>,
    universe: &Universe, 
    center_postions: &Vec<Vec3>,
    mut rng: &mut ChaCha8Rng
) -> Option<Vec3>
{
    if local_graph.node_count() > 0 {
        //If unlucky, no extension
        if rng.gen_range(1..=100) > universe.blob_combo_chance {
            return None;
        }
        // return None;

        println!("IT'S HAPPENING, WOOOOOOOOOOOOOOOOOO");

        //Find one random previous center or extension center
        //Use it to grow another part of the blob
        let origin_pos = local_graph
            .node_weights()
            .filter(|x| (x.role == NodeType::Center) || (x.role == NodeType::Extension))
            .map(|x| x.clone())
            .collect::<Vec<NodeData>>()
            .choose(rng)
            .unwrap()
            .get_vec();

        let mut extension_pos;
        loop {
            extension_pos = extend_blob(
                rng, 
                universe, 
                BlobType::Disc, 
                origin_pos
            );
            let blob_clipping = is_blob_clipping(
                center_postions, 
                extension_pos, 
                universe.disc_extension_distance
            );
            if blob_clipping == false {
                break;
            }
        }
        return Some(extension_pos);
    } else {
        let mut origin_pos;
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
        return Some(origin_pos);
    }
}


//RECURSIVE
fn place_members(
    mut local_graph: UnGraph<NodeData, EdgeData>,
    universe: &Universe, 
    center_postions: &mut Vec<Vec3>,
    rng: &mut ChaCha8Rng
) -> UnGraph<NodeData, EdgeData>
{
    //TODO: FIX
    let radius: f32 = universe.disc_radius;
    let height: f32 = universe.disc_height;

    //Find one random previous center or extension center
    //Use it to grow another part of the blob
    //If this is first, gets random pos in universe
    //If the chance for extension doesn't trigger, it returns
    let origin_pos = get_start_pos(&local_graph, universe, &center_postions, rng);
    let origin_pos = match origin_pos {
        Some(v) => v,
        None => return local_graph
    };
    center_postions.push(origin_pos);

    let mut origin_data = NodeData::from(origin_pos);
    origin_data.color = match local_graph.node_count() {
        0 => Color::GOLD,
        _ => Color::BLUE
    };
    origin_data.role = match local_graph.node_count() {
        0 => NodeType::Center,
        _ => NodeType::Extension
    };
    local_graph.add_node(origin_data);

    for _ in 0..universe.n_nodes-1 {
        //Check that no other indices are close, then try again
        loop {
            let member_pos = node_utils::rand_position(radius, height, origin_pos, rng);
            if is_member_clipping(&local_graph, &member_pos, universe.no_no_distance) == false {
                local_graph.add_node(NodeData::from(member_pos));
                break;
            }   
        }

    }

    return place_members(local_graph, universe, center_postions, rng);
}

pub fn generate_disc_blob(
    universe: &Universe, 
    center_postions: &mut Vec<Vec3>,
    rng: &mut ChaCha8Rng
) -> UnGraph<NodeData, EdgeData>
{
    let mut local_graph = UnGraph::<NodeData, EdgeData>::new_undirected();
    
    local_graph = place_members(local_graph, universe, center_postions, rng);
    local_graph = node_utils::calculate_blob_proximity(local_graph, rng);
    local_graph = node_utils::connect_members(
        local_graph, 
        rng, 
        universe.n_member_candidates,
        universe.fluff_requirement
    );

    return local_graph;
}