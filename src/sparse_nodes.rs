use bevy::math::Vec3;
use bevy::prelude::Color;
use petgraph::graph::{NodeIndex, UnGraph};
use rand_chacha::ChaCha8Rng;

use crate::data::*;
use crate::node_utils::{get_sorted_distances, is_member_clipping, rand_disc_position};

fn get_sparse_pos(
    //Mut ref because previous function uses mut ref
    graph: &mut UnGraph<NodeData, EdgeData>,
    rng: &mut ChaCha8Rng,
    universe: &Universe
) -> Vec3 {
    let mut sparse_pos;
    loop {
        sparse_pos = rand_disc_position(
            universe.size.radius, 
            universe.size.height, 
            Vec3::ZERO, 
            rng
        );

        let sparse_clipping = is_member_clipping(
            &graph, 
            &sparse_pos, 
            universe.sparse_distance_tolerance
        );
        if sparse_clipping == false {
            break;
        }
    }

    return sparse_pos;
}

fn place_sparse_node(
    graph: &mut UnGraph<NodeData, EdgeData>,
    rng: &mut ChaCha8Rng,
    universe: &Universe
) -> bool {
    let sparse_pos = get_sparse_pos(graph, rng, universe);

    let mut sparse_data = NodeData::default_with_idx(sparse_pos, usize::MAX);
    sparse_data.color = Color::PURPLE;
    sparse_data.role = NodeType::Sparse;
    

    for end_idx in graph.node_indices() {
        let end_node = graph.node_weight(end_idx).unwrap();
        let end_pos = Vec3::new(end_node.x, end_node.y, end_node.z);

        let distance = sparse_pos.distance(end_pos);
        
        sparse_data.neighbor_distances.insert(end_idx, distance);
    }

    let mut unique_connections: Vec<usize> = Vec::new();
    let candidates = &sparse_data.neighbor_distances;
    let candidates = &mut get_sorted_distances(candidates);
    let candidates: Vec<(NodeIndex, f32)> = candidates[0..universe.n_sparse_connections]
        .iter()
        .map(|x| *x)
        .filter(|x| {
            let candidate_node = graph.node_weight(x.0).unwrap();
            dbg!(candidate_node.blob_idx);
            if !(unique_connections.contains(&candidate_node.blob_idx)) {
                unique_connections.push(candidate_node.blob_idx);
                
            }
            return graph.edges(x.0).count() <= candidate_node.n_connections;
        })
        .collect();

    //Try again if node is only connected to one type of blob
    let contains_sparse = unique_connections.contains(&usize::MAX);
    if contains_sparse == false && unique_connections.len() <= 1 {
        return false;
    }

    let sparse_idx = graph.add_node(sparse_data);  
    for (candidate_idx, candidate_distance) in candidates {
        graph.add_edge(
            sparse_idx, 
            candidate_idx, 
            EdgeData::with_color(candidate_distance, Color::PURPLE)
        );
        println!("Sparse edge between {:?} and {:?}", sparse_idx, candidate_idx);
    }

    return true;
}

pub fn add_sparse_nodes(
    mut graph: UnGraph<NodeData, EdgeData>,
    rng: &mut ChaCha8Rng,
    universe: &Universe
) -> UnGraph<NodeData, EdgeData> 
{
    //Used to be passed to get_sparse_pos
    // let origin_pos = Vec3::ZERO;

    for _ in 0..universe.n_sparse_nodes {
        loop {
            //Will return true if succesful in placing
            match place_sparse_node(&mut graph, rng, universe) {
                true => break,
                false => continue
            }
        }
    }

    return graph;
}