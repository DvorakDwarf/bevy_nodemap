use bevy::math::Vec3;
use bevy::prelude::Color;
use petgraph::algo;
use petgraph::graph::{Graph, NodeIndex};
use rand;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::data::{EdgeData, NodeData, NodeType, Universe};
use crate::node_utils::{get_sorted_distances, is_blob_connected};

fn calculate_3d_distance(start_node: &NodeData, end_node: &NodeData) -> f32{
    let distance = (
        (end_node.x - start_node.x).powf(2.0) + 
        (end_node.y - start_node.y).powf(2.0) +
        (end_node.z - start_node.z).powf(2.0)
    ).sqrt();

    return distance;
}

pub fn calculate_outer_distances(mut graph: Graph::<NodeData, EdgeData>) 
    -> Graph::<NodeData, EdgeData> {

    //Borrow checker fighting
    let immutable_graph = graph.clone();

    for start_idx in graph.node_indices() {
        let mut start_node = graph.node_weight_mut(start_idx).unwrap();

        for end_idx in immutable_graph.node_indices() {
            if start_idx == end_idx {
                continue;
            } else if start_node.neighbor_distances.contains_key(&end_idx) {
                continue;
            }

            let end_node = immutable_graph.node_weight(end_idx).unwrap();

            let distance = calculate_3d_distance(start_node, end_node);
            
            start_node.outer_distances.insert(end_idx, distance);
        }
    }

    return graph;
}

fn get_centers(graph: &Graph::<NodeData, EdgeData>) -> Vec<(NodeIndex, &NodeData)> {
    let centres = graph.node_indices()
        .map(|idx| (idx, graph.node_weight(idx).unwrap()))
        .filter(|x| x.1.role == NodeType::Center)
        .collect();

    return centres;
}

fn connect_blob(
    mut graph: Graph::<NodeData, EdgeData>, 
    rng: &mut ChaCha8Rng,
    n_interblob_edges: usize,
    idx_1: NodeIndex, 
    idx_2: NodeIndex) -> Graph<NodeData, EdgeData> 
{
    //Kinda double work but idc, cleaner arguments
    let center_1 = graph.node_weight(idx_1).unwrap();
    let center_2 = graph.node_weight(idx_2).unwrap();

    //All nodes belonging to a blob
    let members_1: Vec<NodeIndex> = center_1.neighbor_distances
        .iter()
        .map(|x| *x.0)
        .collect();

    let members_2: Vec<NodeIndex> = center_2.neighbor_distances
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
    distances.shuffle(rng);

    for connection in distances {
        graph.update_edge(
            connection.0, 
            connection.1, 
            EdgeData::with_color(connection.2, Color::TOMATO)
        );
    }

    return graph; 
}

fn pick_blob_couples(
    mut graph: Graph::<NodeData, EdgeData>,
    blob_order: &Vec<(NodeIndex, NodeData)>,
    centre_indices: &Vec<NodeIndex>,
    rng: &mut ChaCha8Rng,
    n_blob_candidates: usize) -> (Graph<NodeData, EdgeData>, bool) 
{
    for (start_idx, centre_node) in blob_order {
        println!("Connecting blob {:?}", start_idx);

        //Same idea as with blob_order
        //Make sure it's sorted to get the closest nodes
        //Remove any non-center nodes
        let candidates = &centre_node.outer_distances;
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

pub fn connect_blobs(
    mut graph: Graph::<NodeData, EdgeData>, 
    rng: &mut ChaCha8Rng,
    universe: &Universe) -> Graph::<NodeData, EdgeData> 
{
    //Make sure each one is reached at least once in a full run
    //DANGEROUS: THE NODEDATA MIGHT BECOME OUTDATED
    let mut blob_order: Vec<(NodeIndex, NodeData)> = get_centers(&graph)
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
    center_postions: &Vec<Vec3>, 
    origin_pos: Vec3,
    blob_distance_tolerence: f32) -> bool 
{
    for position in center_postions {
        if origin_pos.distance(*position) < blob_distance_tolerence {
            return true;
        }
    }

    return false;
}