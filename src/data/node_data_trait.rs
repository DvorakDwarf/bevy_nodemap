use std::{collections::HashMap, fmt::Debug, usize};
use petgraph::graph::NodeIndex;
use bevy::prelude::*;
use bevy::color::palettes::css;

use super::NodeType;

#[derive(Debug, Clone)]
pub struct GraphData {
    pub pos: Vec3,
    pub color: Srgba,
    pub blob_idx: usize,
    pub role: NodeType,
    pub n_connections: usize,
    pub neighbor_distances: HashMap<NodeIndex, f32>,
    pub outer_distances: HashMap<NodeIndex, f32>,
    // pub entity_id: Entity //The entity associated with this node
}

impl Default for GraphData {
    fn default() -> GraphData {
         return GraphData {
            pos: Vec3::ZERO,
            color: css::RED,
            blob_idx: 0,
            role: NodeType::Member,
            n_connections: 999,
            neighbor_distances: HashMap::new(),
            outer_distances: HashMap::new()
         };
    }
}

pub trait NodeData : Send + Sync + Clone {
    fn default_with_idx(vec: Vec3, blob_idx: usize) -> Self;
    fn get_graph_data(&self) -> &GraphData;
    fn get_mut_graph_data(&mut self) -> &mut GraphData;
    //remove this eventually
    fn get_vec(&self) -> Vec3 {
        return self.get_graph_data().pos;
    }
}

// impl Clone for dyn NodeData {
//     fn clone(&self) -> Self { todo!() }
// }

// impl Clone for Box<dyn NodeData> {
//     fn clone(&self) -> Self { todo!() }

// }

// impl Debug for dyn NodeData {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }