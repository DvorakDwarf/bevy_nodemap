use core::fmt::Debug;

use bevy::{prelude::*, render::render_graph::Node};
use petgraph::graph::UnGraph;

use super::{EdgeData, NodeData};

#[derive(Debug, Resource)]
pub struct GraphState<N: NodeData> {
    pub graph: UnGraph<N, EdgeData>
}

impl<N: NodeData> GraphState<N> {
    pub fn new(graph: UnGraph<N, EdgeData>) -> GraphState<N> {
        return GraphState {
            graph
        };
    }
}