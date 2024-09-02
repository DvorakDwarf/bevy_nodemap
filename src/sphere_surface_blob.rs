//TODO:
//PUT THIS IN A MODULE WITH OTHER BLOB TYPES

use std::f32::consts::PI;

use petgraph::graph::UnGraph;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use bevy::prelude::*;

use crate::data::*;
use crate::node_utils;

#[derive(Debug)]
pub struct DiscBlob {
    pub n_nodes: usize,
    pub n_member_candidates: usize,
    pub fluff_requirement: f32,
    pub combo_chance: usize,
    pub no_no_distance: f32,
    pub radius: f32,
    pub height: f32
}

impl Blob for DiscBlob {
    fn get_combo_chance(&self) -> usize {
        return self.combo_chance;
    }
    fn get_n_nodes(&self) -> usize {
        return self.n_nodes;
    }
    fn get_no_no_distance(&self) -> f32 {
        return self.no_no_distance;
    }

    fn rand_position(
        &self,
        origin_pos: Vec3, 
        rng: &mut ChaCha8Rng) -> Vec3 {
    
        let theta: f32 = rng.gen_range(0.0..2.0*PI);
    
        //Why is sqrt there ? I notice it makes it
        //more circular and more spread apart from the center
        let x = ((rng.gen::<f32>().sqrt() * self.radius) * theta.cos()) + origin_pos.x;
        let y = rng.gen_range(0.0..self.height) + origin_pos.y; 
        let z = ((rng.gen::<f32>().sqrt() * self.radius) * theta.sin()) + origin_pos.z;
    
        let new_pos = Vec3::new(x, y, z);
    
        return new_pos;
    }

    fn generate_blob(
        &self,
        universe: &Universe, 
        locations: &mut Vec<Location>,
        rng: &mut ChaCha8Rng
    ) -> UnGraph<NodeData, EdgeData>
    {
        let mut local_graph = UnGraph::<NodeData, EdgeData>::new_undirected();
        
        local_graph = self.place_members(local_graph, universe, locations, rng);
        local_graph = node_utils::calculate_blob_proximity(local_graph, rng);
        local_graph = node_utils::connect_members(
            local_graph, 
            rng, 
            self.n_member_candidates,
            self.fluff_requirement
        );
    
        return local_graph;
    }
}