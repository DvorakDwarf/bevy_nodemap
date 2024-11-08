use core::fmt::Debug;

use bevy::prelude::*;
use petgraph::graph::UnGraph;
use rand_chacha::ChaCha8Rng;
use rand::prelude::SliceRandom;
use rand::Rng;

use crate::blob_utils::is_blob_clipping;
use crate::node_utils::is_member_clipping;
use crate::node_utils::random_disc_easing_pos;

use super::Location;
use super::LocationType;
use super::NodeType;
use super::Universe;
use super::{EdgeData, NodeData};

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

    fn get_start_pos<N: NodeData, B: Blob> (  
        &self,  
        local_graph: &UnGraph<N, EdgeData>,
        locations: &Vec<Location>,
        mut rng: &mut ChaCha8Rng,
        universe: &Universe<B>
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
                .filter(|x| (x.get_graph_data().role == NodeType::Center) || (x.get_graph_data().role == NodeType::Extension))
                .map(|x| x.clone())
                .collect::<Vec<&N>>()
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
                dbg!(locations);
                let blobs_spawned = locations.iter()
                    .filter(|x| x.location_type == LocationType::Blob)
                    .collect::<Vec<&Location>>()
                    .len();
                dbg!(blobs_spawned);
                let time = (universe.n_blobs as f32) / blobs_spawned as f32;

                //Random GLOBAL position
                origin_pos = random_disc_easing_pos(
                    universe.size.radius,
                    universe.size.height,
                    Vec3::new(0.0, 0.0, 0.0), 
                    simple_easing::circ_out,
                    time,
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
    fn place_members<N: NodeData, B: Blob> (
        &self,
        mut local_graph: UnGraph<N, EdgeData>,
        universe: &Universe<B>, 
        locations: &mut Vec<Location>,
        rng: &mut ChaCha8Rng,
        blob_idx: usize
    ) -> UnGraph<N, EdgeData>
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
            location_type: LocationType::Blob,
            center_pos: origin_pos,
            distance_tolerance: universe.blob_distance_tolerance
        });
    
        let mut origin_data = N::default_with_idx(origin_pos, blob_idx);
        origin_data.get_graph_data().color = match local_graph.node_count() {
            0 => Color::GOLD,
            _ => Color::BLUE
        };
        origin_data.get_graph_data().role = match local_graph.node_count() {
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
                    local_graph.add_node(NodeData::default_with_idx(member_pos, blob_idx));
                    break;
                }   
            }
    
        }
    
        return self.place_members(local_graph, universe, locations, rng, blob_idx);
    }

    fn generate_blob<N: NodeData, B: Blob>(
        &self,
        universe: &Universe<B>, 
        locations: &mut Vec<Location>,
        rng: &mut ChaCha8Rng,
        blob_idx: usize
    ) -> UnGraph<N, EdgeData>;
}

// impl Debug for dyn Blob {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }