use bevy::prelude::*;
use petgraph::graph::Graph;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Debug, Default)]
pub struct NodeData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub color: Color,
    pub n_connections: u32
}

impl NodeData {    
    pub fn get_vec(&self) -> Vec3 {
        return Vec3::new(self.x, self.y, self.z)
    }
}

impl From<Vec3> for NodeData {
    fn from(vec: Vec3) -> NodeData {
        let mut rng = ChaCha8Rng::seed_from_u64(1337);
        return NodeData {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            color: Color::RED,
            n_connections: rng.gen_range(0..7)
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

#[derive(Debug)]
pub enum BlobType {
    Disc
}

//TODO: variant will be an array/vector/something with possible variants
//Which are picked randomly or weighted and then randomly picked
//There will also be some amount of nodes allocated per blob in a range
pub struct Universe {
    pub n_nodes: usize,
    pub no_no_distance: f64,
    pub blob_variant: BlobType
}