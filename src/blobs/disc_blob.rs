use petgraph::graph::UnGraph;
use rand_chacha::ChaCha8Rng;
use bevy::prelude::*;

use crate::data::*;
use crate::node_utils;
use crate::node_utils::rand_disc_position;

#[derive(Debug)]
pub struct DiscBlob {
    pub radius: f32,
    pub height: f32,
    pub extension_radius: f32,
    pub n_nodes: usize,
    pub n_member_candidates: usize,
    pub fluff_requirement: f32,
    pub combo_chance: usize,
    pub no_no_distance: f32
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
    fn get_extension_distance(&self) -> f32 {
        return self.extension_radius;
    }

    fn rand_position(
        &self,
        origin_pos: Vec3, 
        rng: &mut ChaCha8Rng) -> Vec3 
    {
        return rand_disc_position(
            self.radius, self.height, origin_pos, rng
        );
    }

    fn rand_extension_position(
        &self,
        origin_pos: Vec3, 
        rng: &mut ChaCha8Rng) -> Vec3 
    {
        return rand_disc_position(
            self.radius * 1.0, self.height * 1.0, origin_pos, rng
        );
    }

    fn generate_blob<N: NodeData + Clone>(
        &self,
        universe: &Universe, 
        locations: &mut Vec<Location>,
        rng: &mut ChaCha8Rng,
        blob_idx: usize
    ) -> UnGraph<N, EdgeData>
    {
        dbg!("Start");
        let mut local_graph = UnGraph::<N, EdgeData>::new_undirected();
        
        local_graph = self.place_members(local_graph, universe, locations, rng, blob_idx);
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