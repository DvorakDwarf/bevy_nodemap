use std::f32::consts::PI;

use bevy::utils::dbg;
use petgraph::algo;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::IntoNodeReferences;
use rand::{self, Rng};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use bevy::prelude::*;

use crate::data::{BlobType, EdgeData, NodeData, Universe};

fn calculate_blob_proximity(mut graph: Graph::<NodeData, EdgeData>) 
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
            
            start_node.neighbor_distances.push((end_idx, distance));
        }
    }

    return graph;
}

fn is_blob_connected(graph: &Graph::<NodeData, EdgeData>) -> bool {
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
                    println!("Found path !!1!!!! Yipieee");
                }
                false => {
                    println!("There was no path :(");
                    return false;
                },
            }
        }
    }

    return true;
}

fn generate_disc_blob(universe: &Universe, mut rng: ChaCha8Rng) 
    -> Graph::<NodeData, EdgeData> {
    let mut graph = Graph::<NodeData, EdgeData>::new();
    
    //TODO: Arguments-to-be
    let radius: f32 = 10.0;
    let height: f32 = 5.0;

    //Create the first blob origin
    //TODO: Place the origin in a random location  
    let origin_pos = Vec3::new(0.0, 0.0, 0.0);
    let mut origin_data = NodeData::from(origin_pos);
    origin_data.color = Color::BLUE;
    graph.add_node(origin_data);

    for _ in 0..universe.n_nodes {
        //TODO: Check that no other indices are close, then try again

        let theta: f32 = rng.gen_range(0.0..2.0*PI);

        //TODO: Why is sqrt there ?
        let x = (rng.gen::<f32>().sqrt() * radius) * theta.cos() + origin_pos.x;
        let y = rng.gen_range(0.0..height); 
        let z = (rng.gen::<f32>().sqrt() * radius) * theta.sin() + origin_pos.y;

        let member_pos = Vec3::new(x, y, z);

        graph.add_node(NodeData::from(member_pos));
    }

    graph = calculate_blob_proximity(graph);

    return graph;
}

pub fn generate_graph(universe: Universe) -> Graph::<NodeData, EdgeData> {
    let mut rng = ChaCha8Rng::seed_from_u64(1337);

    match universe.blob_variant {
        BlobType::Disc => return generate_disc_blob(&universe, rng),
    }
}