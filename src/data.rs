use bevy::prelude::*;
use petgraph::graph::Graph;

#[derive(Debug, Default)]
pub struct NodeData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub color: Color
}

impl NodeData {
    pub fn new(x: f32, y: f32, z: f32) -> NodeData {
        return NodeData {
            x, y, z,
            color: Color::RED,
        };
    }
    
    pub fn get_vec(&self) -> Vec3 {
        return Vec3::new(self.x, self.y, self.z)
    }
}

impl From<Vec3> for NodeData {
    fn from(vec: Vec3) -> NodeData {
        return NodeData {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            color: Color::RED
        }
    }
}

#[derive(Debug, Default)]
pub struct EdgeData {
    length: f32
}

impl EdgeData {
    // pub fn new(length: f32) -> EdgeData {
    //     return EdgeData {
    //         length
    //     };
    // }
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