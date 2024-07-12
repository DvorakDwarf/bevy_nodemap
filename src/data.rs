use bevy::prelude::*;
use petgraph::graph::Graph;

#[derive(Debug, Default)]
pub struct NodeData {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl NodeData {
    pub fn new(x: f32, y: f32, z: f32) -> NodeData {
        return NodeData {
            x, y, z
        };
    }
}

#[derive(Debug, Default)]
pub struct EdgeData {
    length: f32
}

impl EdgeData {
    pub fn new(length: f32) -> EdgeData {
        return EdgeData {
            length
        };
    }
}

#[derive(Resource)]
pub struct GlobalState {
    pub graph: Graph<NodeData, EdgeData>
}

impl GlobalState {
    pub fn new(graph: Graph<NodeData, EdgeData>) -> GlobalState {
        return GlobalState {
            graph
        };
    }
}