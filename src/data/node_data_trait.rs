use std::{collections::HashMap, usize};
use petgraph::graph::NodeIndex;
use bevy::prelude::*;

use super::NodeType;

#[derive(Debug, Clone)]
pub struct GraphData {
    pub pos: Vec3,
    pub color: Color,
    pub blob_idx: usize,
    pub role: NodeType,
    pub n_connections: usize,
    pub neighbor_distances: HashMap<NodeIndex, f32>,
    pub outer_distances: HashMap<NodeIndex, f32>
}

pub trait NodeData {
    fn get_graph_data(&self) -> GraphData;
    fn get_vec(&self) -> Vec3 {
        return self.get_graph_data().pos;
    }
    fn default_with_idx(vec: Vec3, blob_idx: usize) -> Self;
}

// impl Clone for dyn NodeData {
//     fn clone(&self) -> Self { todo!() }
// }