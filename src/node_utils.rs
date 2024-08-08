use std::collections::HashMap;
use std::f32::consts::PI;
use petgraph::algo;
use petgraph::graph::{Graph, NodeIndex};
use rand::{self, Rng};
use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;
use bevy::prelude::*;

use crate::data::{EdgeData, NodeData};

pub fn is_blob_connected(graph: &Graph::<NodeData, EdgeData>) -> bool {
    let mut counter = 0;
    let mut connected = true;
    for start_idx in graph.node_indices() {
        for end_idx in graph.node_indices() {
            //Holy shit, this just worked first try, wtf
            let path = algo::has_path_connecting(
                &graph,
                start_idx,               // start
                end_idx,
                None
            );
    
            match path {
                true => {
                    counter += 1;
                    // println!("Found path !!1!!!! Yipieee");
                }
                false => {
                    connected = false;
                },
            }
        }
    }

    println!("{counter} paths found");
    return connected;
}


pub fn calculate_blob_proximity(mut graph: Graph::<NodeData, EdgeData>, rng: &mut ChaCha8Rng) 
    -> Graph<NodeData, EdgeData> {
    //Borrow checked fighting
    let immutable_graph = graph.clone();

    for start_idx in graph.node_indices() {
        let mut start_node = graph.node_weight_mut(start_idx).unwrap();

        for end_idx in immutable_graph.node_indices() {
            if start_idx == end_idx {
                continue;
            }

            let end_node = immutable_graph.node_weight(end_idx).unwrap();

            let distance = (
                (end_node.x - start_node.x).powf(2.0) + 
                (end_node.y - start_node.y).powf(2.0) +
                (end_node.z - start_node.z).powf(2.0)
            ).sqrt();
            
            start_node.neighbor_distances.insert(end_idx, distance);
        }
    }

    return graph;
}

pub fn get_sorted_distances(map: &HashMap<NodeIndex, f32>) -> Vec<(NodeIndex, f32)> {
    //Sort based on distance, ascending
    let mut distances_list: Vec<(NodeIndex, f32)> = map
        .iter()
        .map(|x| (*x.0, *x.1))
        .collect();

    distances_list.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    return distances_list;
}

pub fn connect_members(
    mut graph: Graph::<NodeData, EdgeData>, 
    rng: &mut ChaCha8Rng,
    n_member_candidates: usize) -> Graph<NodeData, EdgeData> 
{
    //Does repeat work. In fact, a lot of this code does
    let mut stop = false;
    while stop == false {
        //Make sure each one is reached at least once in a full run
        let mut node_order = graph.node_indices().collect::<Vec<NodeIndex>>();
        node_order.shuffle(rng);


        //TODO: TOO MANY EDGES
        //CORRECTION: IT SEEMS THAT WAY BECAUSE LINES GO THROUGH NODES
        //IT WORKS FINE WITH LOW n_candidates BUT I SHOULD FIX THIS
        for start_idx in node_order {
            let mut start_node = graph.node_weight(start_idx).unwrap().clone();
            let n_edges = graph.edges(start_idx).count();
            // dbg!(&n_edges);
            if n_edges > start_node.n_connections {
                // dbg!("Triggered 1");
                continue;
            }

            //Same idea as with node_order
            //Make sure it's sorted to get the closest nodes
            let candidates = &start_node.neighbor_distances;
            let candidates = &mut get_sorted_distances(candidates)[0..n_member_candidates];
            candidates.shuffle(rng);

            for (candidate_idx, candidate_distance) in candidates {
                //Maybe it should mark the fact it tried one candidate already
                let candidate_node = graph.node_weight(*candidate_idx).unwrap();
                let n_edges = graph.edges(*candidate_idx).count();
                if n_edges > candidate_node.n_connections {
                    // dbg!("Triggered 2");
                    continue;
                }

                graph.update_edge(
                    start_idx, 
                    *candidate_idx, 
                    EdgeData::new(*candidate_distance)
                );
                println!("Updated edge between {:?} and {:?}", start_idx, candidate_idx);

                //It stops looking at candidates after one is update to
                //keep it roughly uniformly distributed
                break;
            }

            if is_blob_connected(&graph) == true {
                stop = true;
                break;
            }
        }
    }

    return graph;
}

fn get_positons(graph: &Graph::<NodeData, EdgeData>) -> Vec<Vec3> {
    //Sort based on distance, ascending
    let positions_list: Vec<Vec3> = graph
        .node_weights()
        .map(|x| Vec3::new(x.x, x.y, x.z))
        .collect();

    return positions_list;
}


pub fn is_member_clipping(
    graph: &Graph::<NodeData, EdgeData>, 
    member_pos: &Vec3,
    distance_tolerance: f32) -> bool 
{
    let positions = get_positons(graph);
    for position in positions {
        if member_pos.distance(position) < distance_tolerance {
            return true;
        }
    }

    return false;
}

pub fn rand_position(
    radius: f32, 
    height: f32, 
    origin_pos: Vec3, 
    rng: &mut ChaCha8Rng) -> Vec3 {

    let theta: f32 = rng.gen_range(0.0..2.0*PI);

    //Why is sqrt there ? I notice it makes it
    //more circular and more spread apart from the center
    let x = ((rng.gen::<f32>().sqrt() * radius) * theta.cos()) + origin_pos.x;
    let y = rng.gen_range(0.0..height) + origin_pos.y; 
    let z = ((rng.gen::<f32>().sqrt() * radius) * theta.sin()) + origin_pos.z;

    let new_pos = Vec3::new(x, y, z);

    return new_pos;
}