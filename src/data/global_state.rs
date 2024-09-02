use core::fmt::Debug;

use bevy::prelude::*;
use petgraph::graph::UnGraph;

use super::{EdgeData, NodeData};

#[derive(Debug, Resource)]
pub struct GlobalState {
    pub graph: UnGraph<NodeData, EdgeData>
}

impl GlobalState {
    pub fn new(graph: UnGraph<NodeData, EdgeData>) -> GlobalState {
        return GlobalState {
            graph
        };
    }
}