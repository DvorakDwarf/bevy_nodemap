use bevy::math::Vec3;
use bevy::color::palettes::css;
use petgraph::algo;
use petgraph::graph::{NodeIndex, UnGraph};
use rand;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::data::*;
use crate::node_utils::{get_sorted_distances, is_blob_connected};

//TODO: Replace with existing .distance
fn calculate_3d_distance<N: NodeData>(start_node: &N, end_node: &N) -> f32{
    let start_pos = start_node.get_graph_data().pos;
    let end_pos = end_node.get_graph_data().pos;
    let distance = (
        (end_pos.x - start_pos.x).powf(2.0) + 
        (end_pos.y - start_pos.y).powf(2.0) +
        (end_pos.z - start_pos.z).powf(2.0)
    ).sqrt();

    return distance;
}

pub fn calculate_outer_distances<N: NodeData + Clone>(
    mut graph: UnGraph<N, EdgeData>
    ) -> UnGraph<N, EdgeData>
{

    //Borrow checker fighting
    let immutable_graph = graph.clone();

    for start_idx in graph.node_indices() {
        let mut start_node = graph.node_weight_mut(start_idx).unwrap();

        for end_idx in immutable_graph.node_indices() {
            if start_idx == end_idx {
                continue;
            } else if start_node.get_graph_data().neighbor_distances.contains_key(&end_idx) {
                continue;
            }

            let end_node = immutable_graph.node_weight(end_idx).unwrap();

            let distance = calculate_3d_distance(start_node, end_node);
            
            start_node.get_mut_graph_data().outer_distances.insert(end_idx, distance);
        }
    }

    return graph;
}

fn get_centers<N: NodeData> (
    graph: &UnGraph<N, EdgeData>
) -> Vec<(NodeIndex, &N)> 
{
    let centres = graph.node_indices()
        .map(|idx| (idx, graph.node_weight(idx).unwrap()))
        .filter(|x| x.1.get_graph_data().role == NodeType::Center)
        .collect();

    return centres;
}

fn connect_blob<N: NodeData> (
    mut graph: UnGraph<N, EdgeData>, 
    rng: &mut ChaCha8Rng,
    n_interblob_edges: usize,
    idx_1: NodeIndex, 
    idx_2: NodeIndex) -> UnGraph<N, EdgeData> 
{
    //Kinda double work but idc, cleaner arguments
    let center_1 = graph.node_weight(idx_1).unwrap();
    let center_2 = graph.node_weight(idx_2).unwrap();

    //All nodes belonging to a blob
    let members_1: Vec<NodeIndex> = center_1.get_graph_data().neighbor_distances
        .iter()
        .map(|x| *x.0)
        .collect();

    let members_2: Vec<NodeIndex> = center_2.get_graph_data().neighbor_distances
        .iter()
        .map(|x| *x.0)
        .collect();

    let mut distances: Vec<(NodeIndex, NodeIndex, f32)> = Vec::new();
    for start_idx in members_1 {
        let start_node = graph.node_weight(start_idx).unwrap();

        //idk why clone is required here and why members_1 didn't complain
        for end_idx in members_2.clone() {
            let end_node = graph.node_weight(end_idx).unwrap();

            let distance = calculate_3d_distance(start_node, end_node);
            distances.push((start_idx, end_idx, distance));
        }
    }
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let distances = &mut distances[0..n_interblob_edges];
    
    //This is useless unless I pick 3 random from top 5 choices
    // distances.shuffle(rng);

    for connection in distances {
        graph.update_edge(
            connection.0, 
            connection.1, 
            EdgeData::with_color(connection.2, css::TOMATO)
        );
    }

    return graph; 
}

fn pick_blob_couples<N: NodeData> (
    mut graph: UnGraph<N, EdgeData>,
    blob_order: &Vec<(NodeIndex, N)>,
    centre_indices: &Vec<NodeIndex>,
    rng: &mut ChaCha8Rng,
    n_blob_candidates: usize) -> (UnGraph<N, EdgeData>, bool) 
{
    for (start_idx, centre_node) in blob_order {
        println!("Connecting blob {:?}", start_idx);

        //Same idea as with blob_order
        //Make sure it's sorted to get the closest nodes
        //Remove any non-center nodes
        let candidates = &centre_node.get_graph_data().outer_distances;
        let candidates = get_sorted_distances(&candidates);
        let mut candidates: Vec<&(NodeIndex, f32)> = candidates
            .iter()
            .filter(|x| centre_indices.contains(&x.0))
            .collect();
        let candidates = &mut candidates[0..n_blob_candidates];
        candidates.shuffle(rng);

        for (candidate_idx, _) in candidates {
            println!("Considering candidate {:?}", candidate_idx);

            //Holy shit, this just worked first try, wtf
            let path = algo::has_path_connecting(
                &graph,
                *start_idx,
                *candidate_idx,
                None
            );
            if path == true {
                continue;
            }

            let n_interblob_edges = rng.gen_range(1..=4);
            graph = connect_blob(graph, rng, n_interblob_edges, *start_idx, *candidate_idx);

            println!("Updated edge between blobs {:?} and {:?}", start_idx, candidate_idx);

            //It stops looking at candidates after one is updated to
            //keep it roughly uniformly distributed
            break;
        }

        //Might need to limit this to just centres to speed things up
        if is_blob_connected(&graph) == true {
            return (graph, true); 
        }
    }

    return (graph, false);
}

pub fn connect_blobs<N: NodeData + Clone> (
    mut graph: UnGraph<N, EdgeData>, 
    rng: &mut ChaCha8Rng,
    universe: &Universe) -> UnGraph<N, EdgeData>
{
    //Make sure each one is reached at least once in a full run
    //DANGEROUS: THE NODEDATA MIGHT BECOME OUTDATED
    let mut blob_order: Vec<(NodeIndex, N)> = get_centers(&graph)
        .iter()
        .map(|x| (x.0, x.1.clone()))
        .collect();
    blob_order.shuffle(rng);

    let centre_indices: Vec<NodeIndex> = blob_order.iter().map(|x| x.0).collect();

    loop {
        let (updated_graph, stop) = pick_blob_couples(
            graph, 
            &blob_order, 
            &centre_indices, 
            rng,
            universe.n_blob_candidates
        );
        graph = updated_graph;
        if stop == true {
            break;
        }
    }

    return graph;
} 

pub fn is_blob_clipping(
    locations: &Vec<Location>, 
    origin_pos: Vec3,
    blob_distance_tolerance: Option<f32>) -> bool 
{
    //        dbg!(&locations);
    for location in locations {
        let clipping_tolerance = match blob_distance_tolerance {
            Some(override_value) => override_value,
            None => location.distance_tolerance
        };

        if origin_pos.distance(location.center_pos) < clipping_tolerance {
            return true;
        }
    }

    return false;
}

// //RECURSIVE
// pub fn generate_extension_amount(
//     rng: &mut ChaCha8Rng,
//     mut n_centers: usize,
//     blob_combo_chance: usize
// ) {
//     if rng.gen_range(1..=100) <= blob_combo_chance {
//         n_centers += 1;
//         generate_extension_amount(
//             rng, 
//             n_centers, 
//             blob_combo_chance
//         );
//     }
// }