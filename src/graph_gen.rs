use bevy::math::Vec3;
use petgraph::graph::{Graph, NodeIndex, UnGraph};
use petgraph::Undirected;
use rand;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::blob_utils::{calculate_outer_distances, connect_blobs};
use crate::data::{BlobType, EdgeData, NodeData, NodeType, Universe};
use crate::disc_blob;

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

    // dbg!(&graph);
    return graph;
}