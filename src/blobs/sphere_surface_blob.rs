//TODO:
//PUT THIS IN A MODULE WITH OTHER BLOB TYPES

use std::f32::consts::PI;

use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;
use rand_chacha::ChaCha8Rng;
use rand::Rng;
use bevy::prelude::*;

use crate::data::*;
use crate::node_utils;

#[derive(Debug)]
pub struct SphereSurfaceBlob {
    pub radius: f32,
    pub extension_radius: f32,
    pub n_nodes: usize,
    pub n_member_candidates: usize,
    pub fluff_requirement: f32,
    pub combo_chance: usize,
    pub no_no_distance: f32,
}

impl Blob for SphereSurfaceBlob {
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
        //Apparently there's a better way but :shrug:
        let phi: f32 = rng.gen_range(0.0..2.0*PI);
        let costheta: f32 = rng.gen_range(-1.0..1.0);

        let theta = costheta.acos();

        let x = (self.radius * theta.sin() * phi.cos()) + origin_pos.x;
        let y = (self.radius * theta.sin() * phi.sin()) + origin_pos.y;
        let z = (self.radius * theta.cos()) + origin_pos.z;

        let new_pos = Vec3::new(x, y, z);
    
        return new_pos;
    }

    fn rand_extension_position(
        &self,
        origin_pos: Vec3, 
        rng: &mut ChaCha8Rng
    ) -> Vec3 
    {
        return self.rand_position(origin_pos, rng) * 1.0;
    }

    fn generate_blob<N: NodeData + Clone>(
        &self,
        universe: &Universe, 
        locations: &mut Vec<Location>,
        rng: &mut ChaCha8Rng,
        blob_idx: usize
    ) -> UnGraph<N, EdgeData>
    {
        let mut local_graph = UnGraph::<N, EdgeData>::new_undirected();
        
        local_graph = self.place_members(local_graph, universe, locations, rng, blob_idx);        
        local_graph = node_utils::calculate_blob_proximity(local_graph, rng);
        local_graph = node_utils::connect_members(
            local_graph, 
            rng, 
            self.n_member_candidates,
            self.fluff_requirement
        );
    
        // //UGLY
        // let local_graph = local_graph.map(|_, node_data| {
        //     let mut node_data = node_data.clone();
        //     node_data.get_mut_graph_data().color = match node_data.get_graph_data().color {
        //         Color::RED => Color::SALMON,
        //         Color::GOLD => Color::YELLOW,
        //         Color::BLUE => Color::CYAN,
        //         _ => node_data.get_graph_data().color
        //     };

        //     node_data
        // }, 
        // |_, edge_data| {*edge_data});

        return local_graph;
    }
}