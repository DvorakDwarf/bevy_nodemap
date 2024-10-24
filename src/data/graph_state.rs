use core::fmt::Debug;

use bevy::prelude::*;
use petgraph::graph::UnGraph;

use super::{EdgeData, NodeData};

#[derive(Debug, Resource)]
pub struct GraphState {
    pub graph: UnGraph<NodeData, EdgeData>
}

impl GraphState {
    pub fn new(graph: UnGraph<NodeData, EdgeData>) -> GraphState {
        return GraphState {
            graph
        };
    }
}