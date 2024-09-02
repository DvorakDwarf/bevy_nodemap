use std::collections::HashMap;
use core::fmt::Debug;

use bevy::prelude::*;
use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;
use rand_chacha::ChaCha8Rng;
use rand::prelude::SliceRandom;
use rand::Rng;

use crate::blob_utils::is_blob_clipping;
use crate::node_utils::is_member_clipping;
use crate::node_utils::rand_disc_position;

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

// DESCRIBED HOW TO DO PRETTY HERE https://www.karlsims.com/random-in-sphere.html
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
    Sphere
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

//TODO: variant will be an array/vector/something with possible variants
//Which are picked randomly or weighted and then randomly picked
//There will also be some amount of nodes allocated per blob in a range
#[derive(Debug)]
pub struct Universe {
    pub n_blobs: usize,
    //TODO: Not confident what this entails
    //Each blob variant is a different struct with trait Blob
    pub blob_variants: Vec<Box<dyn Blob>>, 
    pub size: UniverseSize,
    pub blob_distance_tolerance: f32,
    pub n_blob_candidates: usize,
    pub min_connections: usize, //TODO: not actually used
    pub max_connections: usize, //TODO: not actually use
    pub n_sparse_nodes: usize,
    pub sparse_distance_tolerance: f32,
    pub n_sparse_connections: usize,
}

#[derive(Debug)]
pub struct UniverseSize {
    pub radius: f32,
    pub height: f32
}

//I don't know if this is good or bad practice
//All the getter method stuff is unnerving
//But the alternative is massive code reuse
pub trait Blob {
    fn get_combo_chance(&self) -> usize;
    fn get_n_nodes(&self) -> usize;
    fn get_no_no_distance(&self) -> f32;
    fn get_extension_distance(&self) -> f32;

    fn rand_position(
        &self,
        origin_pos: Vec3, 
        rng: &mut ChaCha8Rng
    ) -> Vec3;
    fn rand_extension_position(
        &self,
        origin_pos: Vec3, 
        rng: &mut ChaCha8Rng
    ) -> Vec3;

    fn get_start_pos(  
        &self,  
        local_graph: &UnGraph<NodeData, EdgeData>,
        locations: &Vec<Location>,
        mut rng: &mut ChaCha8Rng,
        universe: &Universe
    ) -> Option<Vec3>
    {
        if local_graph.node_count() > 0 {
            //If unlucky, no extension
            if rng.gen_range(1..=100) > self.get_combo_chance() {
                return None;
            }
                
            //Find one random previous center or extension center
            //Use it to grow another part of the blob
            let origin_pos = local_graph
                .node_weights()
                .filter(|x| (x.role == NodeType::Center) || (x.role == NodeType::Extension))
                .map(|x| x.clone())
                .collect::<Vec<NodeData>>()
                .choose(rng)
                .unwrap()
                .get_vec();
    
            let mut extension_pos;
            loop {
                dbg!(origin_pos);
                extension_pos = self.rand_extension_position(
                    origin_pos, 
                    rng
                );
                let blob_clipping = is_blob_clipping(
                    locations, 
                    extension_pos, 
                    Some(self.get_extension_distance())
                );
                dbg!(blob_clipping);

                if blob_clipping == false {
                    break;
                }
            }
            return Some(extension_pos);
        } else {
            let mut origin_pos;
            loop {
                //Random GLOBAL position
                origin_pos = rand_disc_position(
                    universe.size.radius,
                    universe.size.height,
                    Vec3::new(0.0, 0.0, 0.0), 
                    &mut rng
                );
                dbg!(origin_pos);

                let blob_clipping = is_blob_clipping(
                    locations, 
                    origin_pos, 
                    None
                );
                dbg!(blob_clipping);

                if blob_clipping == false {
                    break;
                }
            }
            return Some(origin_pos);
        }
    }

    //RECURSIVE
    fn place_members(
        &self,
        mut local_graph: UnGraph<NodeData, EdgeData>,
        universe: &Universe, 
        locations: &mut Vec<Location>,
        rng: &mut ChaCha8Rng
    ) -> UnGraph<NodeData, EdgeData>
    {
        //Find one random previous center or extension center
        //Use it to grow another part of the blob
        //If this is first, gets random pos in universe
        //If the chance for extension doesn't trigger, it returns
        let origin_pos = self.get_start_pos(&local_graph, locations, rng, universe);
        let origin_pos = match origin_pos {
            Some(v) => v,
            None => return local_graph
        };
    
        //Update locations
        locations.push(Location {
            location_type: LocationType::Blob(BlobType::Disc),
            center_pos: origin_pos,
            distance_tolerance: universe.blob_distance_tolerance
        });
    
        let mut origin_data = NodeData::from(origin_pos);
        origin_data.color = match local_graph.node_count() {
            0 => Color::GOLD,
            _ => Color::BLUE
        };
        origin_data.role = match local_graph.node_count() {
            0 => NodeType::Center,
            _ => NodeType::Extension
        };
        local_graph.add_node(origin_data);
    
        for _ in 0..self.get_n_nodes()-1 {
            //Check that no other indices are close, then try again
            loop {
                let member_pos = self.rand_position(origin_pos, rng);
                let member_clipping = is_member_clipping(
                    &local_graph, &member_pos, self.get_no_no_distance()
                );
                if member_clipping == false {
                    local_graph.add_node(NodeData::from(member_pos));
                    break;
                }   
            }
    
        }
    
        return self.place_members(local_graph, universe, locations, rng);
    }

    fn generate_blob(
        &self,
        universe: &Universe, 
        locations: &mut Vec<Location>,
        rng: &mut ChaCha8Rng
    ) -> UnGraph<NodeData, EdgeData>;
}

impl Debug for dyn Blob {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}