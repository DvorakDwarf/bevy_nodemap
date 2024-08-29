use std::collections::HashMap;

use bevy::prelude::*;
use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Center, Member, Sparse, Extension
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

// //Here lies my attempt at using a normal distribution to control
// //the number of connections. Do later
// fn gen_n_connections(min: usize, max: usize) -> usize {
//     let mut rng = ChaCha8Rng::seed_from_u64(1337);

//     //Normal in spirit
//     let normal_sample = thread_rng().gen_range(min..max);
//     dbg!(normal_sample);
//     return normal_sample;
// }

//TODO: Randomize
impl From<Vec3> for NodeData {
    fn from(vec: Vec3) -> NodeData {
        // let n_connections = thread_rng().gen_range(2..6);
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
    pub length: f32,
    pub color: Color
}

impl EdgeData {
    pub fn new(length: f32) -> EdgeData {
        let color = if length >  50.0 {
            Color::PURPLE
        } else {
            Color::WHITE
        };

        return EdgeData {
            length,
            color
        };
    }

    pub fn with_color(length: f32, color: Color) -> EdgeData {
        return EdgeData {
            length,
            color
        };
    }
}

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

#[derive(Debug)]
pub enum BlobType {
    Disc
}

#[derive(Debug)]
pub enum VoidType {
    Circle
}

#[derive(Debug)]
pub enum LocationType {
    Blob(BlobType),
    Void(VoidType)
}

#[derive(Debug)]
pub struct Location {
    pub location_type: LocationType,
    pub center_pos: Vec3,
    pub distance_tolerance: f32
}

// impl Location {
//     fn new(center_pos: Vec3, location_type: LocationType) -> Location {
//         return Location {
//             location_type,
//             center_pos,
//             distance_tolerance
//         };
//     }
// }

//TODO: variant will be an array/vector/something with possible variants
//Which are picked randomly or weighted and then randomly picked
//There will also be some amount of nodes allocated per blob in a range
#[derive(Debug)]
pub struct Universe {
    pub n_nodes: usize,
    pub n_blobs: usize,
    pub blob_variant: BlobType,
    pub size: UniverseSize,
    pub no_no_distance: f32,
    pub blob_distance_tolerance: f32,
    pub n_blob_candidates: usize,
    pub n_member_candidates: usize,
    pub fluff_requirement: f32,
    pub min_connections: usize, //TODO: not actually used
    pub max_connections: usize, //TODO: not actually use
    pub n_sparse_nodes: usize,
    pub sparse_distance_tolerance: f32,
    pub n_sparse_connections: usize,
    pub blob_combo_chance: usize,
    pub disc_radius: f32,
    pub disc_height: f32,
    pub disc_extension_distance: f32
}

#[derive(Debug)]
pub struct UniverseSize {
    pub radius: f32,
    pub height: f32
}