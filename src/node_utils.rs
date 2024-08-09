use std::collections::HashMap;
use std::f32::consts::PI;
use petgraph::{algo, Undirected};
use petgraph::graph::{Graph, NodeIndex, UnGraph};
use rand::{self, Rng};
use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;
use bevy::prelude::*;

use crate::data::{EdgeData, NodeData};

pub fn is_blob_connected(
    graph: &UnGraph<NodeData, EdgeData>) -> bool 
{
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

    // println!("{counter} paths found");
    return connected;
}


pub fn calculate_blob_proximity(
    mut graph: UnGraph<NodeData, EdgeData>, 
    rng: &mut ChaCha8Rng) -> UnGraph<NodeData, EdgeData> 
{
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

fn average_connections(graph: &UnGraph<NodeData, EdgeData>) -> f32 {
    let n_nodes = graph.node_count() as f32;

    let total_connections: Vec<usize> = graph
        .node_indices()
        .map(|idx| graph.edges(idx).count()).collect();

    let average = total_connections.iter().sum::<usize>() as f32 / n_nodes;

    return average;
}

fn get_candidates(rng: &mut ChaCha8Rng, start_node: NodeData, n_member_candidates: usize) 
    -> Vec<(NodeIndex, f32)>
{
    //Same idea as with node_order
    //Make sure it's sorted to get the closest nodes
    let candidates = &start_node.neighbor_distances;
    let candidates = &mut get_sorted_distances(candidates)[0..n_member_candidates];
    candidates.shuffle(rng);
    let candidates = candidates.to_vec();

    return candidates;
}

//The skeleton itself is rather boring, add more connections for fun
fn add_blob_fluff(
    mut graph: UnGraph<NodeData, EdgeData>, 
    rng: &mut ChaCha8Rng,
    n_member_candidates: usize,
    fluff_requirement: f32) -> UnGraph<NodeData, EdgeData> 
{
    let mut fluff = average_connections(&graph);

    while fluff < fluff_requirement {
        for start_idx in graph.node_indices() {
            let start_node = graph.node_weight(start_idx).unwrap().clone();

            //Try next one if node full
            let n_edges = graph.edges(start_idx).count();
            if n_edges > start_node.n_connections {
                // dbg!("Triggered 1");
                continue;
            }

            let candidates = get_candidates(rng, start_node, n_member_candidates);
            for (candidate_idx, candidate_distance) in candidates {
                //Maybe it should mark the fact it tried one candidate already
                let candidate_node = graph.node_weight(candidate_idx).unwrap();
                let n_edges = graph.edges(candidate_idx).count();
                if n_edges > candidate_node.n_connections {
                    // dbg!("Triggered 2");
                    continue;
                }
                if graph.find_edge(start_idx, candidate_idx) != None {
                    continue;
                }

                graph.add_edge(
                    start_idx, 
                    candidate_idx, 
                    EdgeData::with_color(candidate_distance, Color::GRAY)
                );
                println!("Added fluff between {:?} and {:?}", start_idx, candidate_idx);

                //It stops looking at candidates after one is updated to
                //keep it roughly uniformly distributed
                break;
            }

            fluff = average_connections(&graph);
            println!("New fluff amount: {}", fluff);

            if fluff >= fluff_requirement {
                break;
            }
        }
    }

    return graph;
}

pub fn connect_members(
    mut graph: UnGraph<NodeData, EdgeData>, 
    rng: &mut ChaCha8Rng,
    n_member_candidates: usize,
    fluff_requirement: f32) -> UnGraph<NodeData, EdgeData> 
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
            let start_node = graph.node_weight(start_idx).unwrap().clone();
            let n_edges = graph.edges(start_idx).count();
            // dbg!(&n_edges);
            if n_edges > start_node.n_connections {
                // dbg!("Triggered 1");
                continue;
            }

            let candidates = get_candidates(rng, start_node, n_member_candidates);

            for (candidate_idx, candidate_distance) in candidates {
                //Maybe it should mark the fact it tried one candidate already
                let candidate_node = graph.node_weight(candidate_idx).unwrap();
                let n_edges = graph.edges(candidate_idx).count();
                if n_edges > candidate_node.n_connections {
                    // dbg!("Triggered 2");
                    continue;
                }

                graph.update_edge(
                    start_idx, 
                    candidate_idx, 
                    EdgeData::new(candidate_distance)
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

    graph = add_blob_fluff(graph, rng, n_member_candidates, fluff_requirement);

    let average_n_connections = average_connections(&graph);
    println!("\n\n\nRed shuffle blob average connections: {}", average_n_connections);
    dbg!(graph.edges(NodeIndex::new(1)).count());
    dbg!(graph.edges(NodeIndex::new(3)).count());
    dbg!(graph.neighbors(NodeIndex::new(4)).count());
    println!("\n\n\n");

    return graph;
}

pub fn connect_members_no_shuffle(
    mut graph: UnGraph<NodeData, EdgeData>, 
    rng: &mut ChaCha8Rng,
    n_member_candidates: usize) -> UnGraph<NodeData, EdgeData> 
{
    //TODO: JUST FOR TESTING
    for node in graph.node_weights_mut() {
        if node.color == Color::BLUE {
            continue;
        }

        node.color = Color::GOLD
    }

    //Does repeat work. In fact, a lot of this code does
    let mut stop = false;
    while stop == false {
        let nodes = graph.node_indices().collect::<Vec<NodeIndex>>();
        let start_idx = *nodes.choose(rng).unwrap();

        let start_node = graph.node_weight(start_idx).unwrap().clone();
        let n_edges = graph.edges(start_idx).count();
        // dbg!(&n_edges);
        if n_edges > start_node.n_connections {
            // dbg!("Triggered 1");
            continue;
        }

        //Go 1-by-1 to not do less repeat work 
        let candidates = get_candidates(rng, start_node, n_member_candidates);

        for (candidate_idx, candidate_distance) in candidates {
            //Maybe it should mark the fact it tried one candidate already
            let candidate_node = graph.node_weight(candidate_idx).unwrap();
            let n_edges = graph.edges(candidate_idx).count();
            if n_edges > candidate_node.n_connections {
                // dbg!("Triggered 2");
                continue;
            }

            graph.update_edge(
                start_idx, 
                candidate_idx, 
                EdgeData::new(candidate_distance)
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

    let average_n_connections = average_connections(&graph);
    println!("\n\n\nGold no shuffle blob average connections: {}\n\n\n", average_n_connections);

    return graph;
}


fn get_positons(graph: &UnGraph<NodeData, EdgeData>) -> Vec<Vec3> {
    //Sort based on distance, ascending
    let positions_list: Vec<Vec3> = graph
        .node_weights()
        .map(|x| Vec3::new(x.x, x.y, x.z))
        .collect();

    return positions_list;
}


pub fn is_member_clipping(
    graph: &UnGraph<NodeData, EdgeData>, 
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