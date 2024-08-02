use std::collections::HashMap;

use bevy::prelude::*;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Center, Member
}

#[derive(Debug, Clone)]
pub struct NodeData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub color: Color,
    pub role: NodeType,
    pub n_connections: usize,
    pub neighbor_distances: HashMap<NodeIndex, f32>,
    pub outer_distances: HashMap<NodeIndex, f32>
}

impl NodeData {    
    pub fn get_vec(&self) -> Vec3 {
        return Vec3::new(self.x, self.y, self.z)
    }
}

//Here lies my attempt at using a normal distribution to control
//the number of connections. Do later
fn gen_n_connections(min: usize, max: usize) -> usize {
    let mut rng = ChaCha8Rng::seed_from_u64(1337);

    //Normal in spirit
    let normal_sample = thread_rng().gen_range(min..max);
    dbg!(normal_sample);
    return normal_sample;
}

//TODO: Randomize
impl From<Vec3> for NodeData {
    fn from(vec: Vec3) -> NodeData {
        //Average: 4 Minimum: 2
        // let n_connections = gen_n_connections(2, 6);
        // let n_connections = thread_rng().gen_range(4..8);

        // dbg!(n_connections);

        return NodeData {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            color: Color::RED,
            role: NodeType::Member,
            n_connections: 999,
            neighbor_distances: HashMap::new(),
            outer_distances: HashMap::new()
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
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

#[derive(Debug, Resource)]
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
#[derive(Debug)]
pub struct Universe {
    pub n_nodes: usize,
    pub n_blobs: usize,
    pub no_no_distance: f64,
    pub blob_variant: BlobType,
    pub size: UniverseSize
}

#[derive(Debug)]
pub struct UniverseSize {
    pub radius: f32,
    pub height: f32
}