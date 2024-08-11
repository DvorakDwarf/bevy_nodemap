use bevy::math::Vec3;
use bevy::prelude::Color;
use petgraph::graph::{Graph, NodeIndex, UnGraph};
use petgraph::Undirected;
use rand;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::blob_utils::{calculate_outer_distances, connect_blobs};
use crate::data::{BlobType, EdgeData, NodeData, NodeType, Universe};
use crate::disc_blob;
use crate::node_utils::{get_sorted_distances, is_member_clipping, rand_position};

fn merge_graphs(
    graph1: &mut UnGraph<NodeData, EdgeData>, 
    graph2: UnGraph<NodeData, EdgeData>) {

    let index_offset = graph1.node_count();

    for node in graph2.node_weights() {
        let mut node = node.clone();

        node.neighbor_distances = node.neighbor_distances.iter().map(|x| {
            let new_idx = x.0.index() + index_offset;
            return (NodeIndex::new(new_idx), *x.1);
        }).collect();

        graph1.add_node(node);
    }

    for edge_idx in graph2.edge_indices() {
        let (source, target) = graph2.edge_endpoints(edge_idx).unwrap();
        let source = NodeIndex::new(source.index() + index_offset);
        let target = NodeIndex::new(target.index() + index_offset);

        let edge = graph2.edge_weight(edge_idx).unwrap();
        graph1.add_edge(source, target, *edge);
    }
} 

fn add_sparse_nodes(
    mut graph: UnGraph<NodeData, EdgeData>,
    rng: &mut ChaCha8Rng,
    universe: &Universe
) -> UnGraph<NodeData, EdgeData> 
{
    let origin_pos = Vec3::ZERO;

    for _ in 0..universe.n_sparse_nodes {
        let mut sparse_pos;
        loop {
            sparse_pos = rand_position(
                universe.size.radius, 
                universe.size.height, 
                origin_pos, 
                rng
            );
    
            let sparse_clipping = is_member_clipping(
                &graph, 
                &sparse_pos, 
                universe.sparse_distance_tolerance
            );
            if sparse_clipping == false {
                break;
            }
        }

        let mut sparse_data = NodeData::from(sparse_pos);
        sparse_data.color = Color::PURPLE;
        sparse_data.role = NodeType::Sparse;
        

        for end_idx in graph.node_indices() {
            let end_node = graph.node_weight(end_idx).unwrap();
            let end_pos = Vec3::new(end_node.x, end_node.y, end_node.z);

            let distance = sparse_pos.distance(end_pos);
            
            sparse_data.neighbor_distances.insert(end_idx, distance);
        }

        let candidates = &sparse_data.neighbor_distances;
        let candidates = &mut get_sorted_distances(candidates);
        let candidates: Vec<(NodeIndex, f32)> = candidates[0..universe.n_sparse_connections]
            .iter()
            .map(|x| *x)
            .filter(|x| {
                let candidate_node = graph.node_weight(x.0).unwrap();
                return graph.edges(x.0).count() <= candidate_node.n_connections;
            })
            .collect();

        let sparse_idx = graph.add_node(sparse_data);  
        
        for (candidate_idx, candidate_distance) in candidates {
            graph.add_edge(
                sparse_idx, 
                candidate_idx, 
                EdgeData::with_color(candidate_distance, Color::PURPLE)
            );
            println!("Sparse edge between {:?} and {:?}", sparse_idx, candidate_idx);
        }
    }

    return graph;
}

pub fn generate_graph(universe: Universe) -> UnGraph<NodeData, EdgeData> {
    let mut rng = ChaCha8Rng::seed_from_u64(1337);
    let mut graph = UnGraph::<NodeData, EdgeData>::new_undirected();
    //Make sure blobs don't spawn too close
    let mut center_positions: Vec<Vec3> = Vec::new();

    //Place blobs
    for _ in 0..universe.n_blobs {
        match universe.blob_variant {
            BlobType::Disc => { 
                let new_blob = disc_blob::generate_disc_blob(
                    &universe, &center_positions, &mut rng
                );
                let new_center = new_blob
                    .node_weights()
                    .find(|x| x.role == NodeType::Center)
                    .unwrap();
                let new_center_pos = Vec3::new(new_center.x, new_center.y, new_center.z);
                center_positions.push(new_center_pos);
                merge_graphs(&mut graph, new_blob);
            },
        }
    }

    //Connect the blobs
    let graph = calculate_outer_distances(graph);
    let graph = connect_blobs(graph, &mut rng, &universe);
    let graph = add_sparse_nodes(graph, &mut rng, &universe);

    // dbg!(&graph);
    return graph;
}