use std::f32::consts::PI;

use petgraph::graph::Graph;
use rand::{self, Rng};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use bevy::prelude::*;

use crate::data::{BlobType, EdgeData, NodeData, Universe};

fn generate_disc_blob(universe: &Universe, mut rng: ChaCha8Rng) -> Graph::<NodeData, EdgeData> {
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

        let theta: f32 = rng.gen_range(0.0..2.0*PI);

        let x = (rng.gen::<f32>().sqrt() * radius) * theta.cos() + origin_pos.x;
        let y = rng.gen_range(0.0..height); 
        let z = (rng.gen::<f32>().sqrt() * radius) * theta.sin() + origin_pos.y;

        let member_pos = Vec3::new(x, y, z);

        graph.add_node(NodeData::from(member_pos));
    }

    return graph;
}

pub fn generate_graph(universe: Universe) -> Graph::<NodeData, EdgeData> {
    let rng = ChaCha8Rng::seed_from_u64(1337);

    match universe.blob_variant {
        BlobType::Disc => return generate_disc_blob(&universe, rng),
    }
}