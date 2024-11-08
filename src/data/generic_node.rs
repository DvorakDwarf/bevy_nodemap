use std::{collections::HashMap, usize};
use core::fmt::Debug;

use bevy::prelude::*;
use petgraph::graph::NodeIndex;

use super::{node_data_trait::GraphData, NodeData, NodeType};

//TODO: pos: Vec3 field instead of x, y, z
#[derive(Debug, Clone)]
pub struct GenericNode {
    pub graph_data: GraphData
}

impl GenericNode {    
    pub fn default_with_idx(vec: Vec3, blob_idx: usize) -> GenericNode {
        // let n_connections = thread_rng().gen_range(2..6);
        // dbg!(n_connections);

        return GenericNode {
            graph_data: GraphData {
                pos: vec,
                color: Color::RED,
                blob_idx: blob_idx, //usize::MAX if sparse. Jank
                role: NodeType::Member,
                n_connections: 999,
                neighbor_distances: HashMap::new(),
                outer_distances: HashMap::new()
            }
        }
    }

    pub fn get_vec(&self) -> Vec3 {
        return Vec3::new(self.x, self.y, self.z)
    }
}

//CLONE !!!
impl NodeData for GenericNode {
    fn get_graph_data(&self) -> GraphData {
        return self.graph_data.clone();
    }
}

// DESCRIBED HOW TO DO PRETTY HERE https://www.karlsims.com/random-in-sphere.html
// //Here lies my attempt at using a normal distribution to control
// //the number of connections. Do later
// fn gen_n_connections(min: usize, max: usize) -> usize {
//     let mut rng = ChaCha8Rng::seed_from_u64(1337);

//     //Normal in spirit
//     let normal_sample = thread_rng().gen_range(min..max);
//     dbg!(normal_sample);
//     return normal_sample;
// }

// impl From<Vec3> for NodeData {
//     fn from(vec: Vec3) -> NodeData {
//         // let n_connections = thread_rng().gen_range(2..6);
//         // dbg!(n_connections);

//         return NodeData {
//             x: vec.x,
//             y: vec.y,
//             z: vec.z,
//             color: Color::RED,
//             blob_idx: usize::MAX, // If unset, max value. Bit jank
//             role: NodeType::Member,
//             n_connections: 999,
//             neighbor_distances: HashMap::new(),
//             outer_distances: HashMap::new()
//         }
//     }
// }