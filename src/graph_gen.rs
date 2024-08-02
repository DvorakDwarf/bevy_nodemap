use petgraph::graph::{Graph, NodeIndex};
use rand;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::data::{BlobType, EdgeData, NodeData, NodeType, Universe};
use crate::disc_blob;

fn merge_graphs(
    graph1: &mut Graph::<NodeData, EdgeData>, 
    graph2: Graph::<NodeData, EdgeData>) {

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

fn calculate_outer_distances(graph: Graph::<NodeData, EdgeData>) 
    -> Graph::<NodeData, EdgeData> {

    // //Borrow checker fighting
    // let immutable_graph = graph.clone();

    // for start_idx in graph.node_indices() {
    //     let mut start_node = graph.node_weight_mut(start_idx).unwrap();

    //     for end_idx in immutable_graph.node_indices() {
    //         if start_idx == end_idx {
    //             continue;
    //         }

    //         let end_node = immutable_graph.node_weight(end_idx).unwrap();

    //         let distance = (
    //             (end_node.x - start_node.x).powf(2.0) + 
    //             (end_node.y - start_node.y).powf(2.0) +
    //             (end_node.z - start_node.z).powf(2.0)
    //         ).sqrt();
            
    //         start_node.neighbor_distances.insert(end_idx, distance);
    //     }
    // }

    return graph;
}

fn get_centers(graph: &Graph::<NodeData, EdgeData>) -> Vec<&NodeData> {
    let centres: Vec<&NodeData> = graph.node_weights()
        .filter(|node| node.role == NodeType::Center)
        .collect();

    return centres;
}

pub fn generate_graph(universe: Universe) -> Graph::<NodeData, EdgeData> {
    let mut rng = ChaCha8Rng::seed_from_u64(1337);
    let mut graph = Graph::<NodeData, EdgeData>::new();

    //Place blobs
    for _ in 0..universe.n_blobs {
        match universe.blob_variant {
            BlobType::Disc => { 
                let new_blob = disc_blob::generate_disc_blob(&universe, &mut rng);
                merge_graphs(&mut graph, new_blob);
            },
        }
    }

    //Connect the blobs
    let graph = calculate_outer_distances(graph);

    //TODO:
    //Put it into format:
    //Vec<(NodeData, Vec<(NodeIndex, f32)>)>
    //Consider raw_nodes ???
    //This is messy
    //Actually scratch this, do one of below
    //Easiest would be to add in an unused field by members
    //Still look at other options

    //Or consider a custom data type
    //Or see if I can put in multiple different structs in
    //Or jut put in a field that's not used unless you are a center

    // let mut centre_distances = Vec::new();
    // for center in centres {
    //     for target in immutable_centres {
    //         centre_distances.push((center, ));
    //     }
    // }

    dbg!(&graph);
    return graph;
}