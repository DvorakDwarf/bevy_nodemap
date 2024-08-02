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

fn calculate_outer_distances(mut graph: Graph::<NodeData, EdgeData>) 
    -> Graph::<NodeData, EdgeData> {

    //Borrow checker fighting
    let immutable_graph = graph.clone();

    for start_idx in graph.node_indices() {
        let mut start_node = graph.node_weight_mut(start_idx).unwrap();

        for end_idx in immutable_graph.node_indices() {
            if start_idx == end_idx {
                continue;
            } else if start_node.neighbor_distances.contains_key(&end_idx) {
                continue;
            }

            let end_node = immutable_graph.node_weight(end_idx).unwrap();

            let distance = (
                (end_node.x - start_node.x).powf(2.0) + 
                (end_node.y - start_node.y).powf(2.0) +
                (end_node.z - start_node.z).powf(2.0)
            ).sqrt();
            
            start_node.outer_distances.insert(end_idx, distance);
        }
    }

    return graph;
}

fn get_centers(graph: &Graph::<NodeData, EdgeData>) -> Vec<(NodeIndex, &NodeData)> {
    // let centres: Vec<&NodeData> = graph.node_weights()
    //     .filter(|node| node.role == NodeType::Center)
    //     .collect();

    let centres = graph.node_indices()
        .map(|idx| (idx, graph.node_weight(idx).unwrap()))
        .filter(|x| x.1.role == NodeType::Center)
        .collect();

    return centres;
}

fn connect_blob() {

}

fn connect_blobs(mut graph: Graph::<NodeData, EdgeData>, rng: &mut ChaCha8Rng)
    -> Graph::<NodeData, EdgeData> {

    //Make sure each one is reached at least once in a full run
    let blob_order = get_centers(&graph);
    blob_order.shuffle(rng);

    //TODO: TOO MANY EDGES
    //CORRECTION: IT SEEMS THAT WAY BECAUSE LINES GO THROUGH NODES
    //IT WORKS FINE WITH LOW n_candidates BUT I SHOULD FIX THIS
    for (start_idx, centre_node) in blob_order {
        // let n_edges = graph.edges(start_idx).count();
        // dbg!(&n_edges);
        // if n_edges > start_node.n_connections {
        //     // dbg!("Triggered 1");
        //     continue;
        // }

        //TODO: CONTINUE FROM HERE, IDENTIFY OTHER CENTRES EITHER IN-LOOP 
        //OR FILTER OUT BEFOREHAND

        //Same idea as with node_order
        //Make sure it's sorted to get the closest nodes
        let candidates = &start_node.neighbor_distances;
        let candidates = &mut get_sorted_distances(candidates)[0..n_candidates];
        candidates.shuffle(rng);

        for (candidate_idx, candidate_distance) in candidates {
            //Maybe it should mark the fact it tried one candidate already
            let candidate_node = graph.node_weight(*candidate_idx).unwrap();
            let n_edges = graph.edges(*candidate_idx).count();
            if n_edges > candidate_node.n_connections {
                // dbg!("Triggered 2");
                continue;
            }

            graph.update_edge(
                start_idx, 
                *candidate_idx, 
                EdgeData::new(*candidate_distance)
            );
            println!("Updated edge between {:?} and {:?}", start_idx, candidate_idx);

            //It stops looking at candidates after one is update to
            //keep it roughly uniformly distributed
            break;
        }

        if is_blob_connected(&graph) == true {
            stop = true;
            break;
        }
    }

    return graph;
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
    let graph = connect_blobs(graph);

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