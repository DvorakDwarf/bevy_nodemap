use bevy::prelude::Color;
use petgraph::algo;
use petgraph::graph::{Graph, NodeIndex};
use rand;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::data::{BlobType, EdgeData, NodeData, NodeType, Universe};
use crate::disc_blob;
use crate::node_utils::{get_sorted_distances, is_blob_connected};

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

fn calculate_3d_distance(start_node: &NodeData, end_node: &NodeData) -> f32{
    let distance = (
        (end_node.x - start_node.x).powf(2.0) + 
        (end_node.y - start_node.y).powf(2.0) +
        (end_node.z - start_node.z).powf(2.0)
    ).sqrt();

    return distance;
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

            let distance = calculate_3d_distance(start_node, end_node);
            
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

fn connect_blob(
    mut graph: Graph::<NodeData, EdgeData>, 
    rng: &mut ChaCha8Rng,
    idx_1: NodeIndex, 
    idx_2: NodeIndex) -> Graph<NodeData, EdgeData> {
    // let candidate_node = graph.node_weight(*candidate_idx).unwrap();
    // let n_edges = graph.edges(*candidate_idx).count();
    // if n_edges > candidate_node.n_connections {
    //     // dbg!("Triggered 2");
    //     continue;
    // }

    //TODO: Arguments-to-be
    let n_interblob_edges = rng.gen_range(1..=4);

    //Kinda double work but idc, cleaner arguments
    let center_1 = graph.node_weight(idx_1).unwrap();
    let center_2 = graph.node_weight(idx_2).unwrap();

    //All nodes belonging to a blob
    let members_1: Vec<NodeIndex> = center_1.neighbor_distances
        .iter()
        .map(|x| *x.0)
        .collect();

    let members_2: Vec<NodeIndex> = center_2.neighbor_distances
        .iter()
        .map(|x| *x.0)
        .collect();

    let mut distances: Vec<(NodeIndex, NodeIndex, f32)> = Vec::new();
    for start_idx in members_1 {
        let start_node = graph.node_weight(start_idx).unwrap();

        //idk why clone is required here and why members_1 didn't complain
        for end_idx in members_2.clone() {
            let end_node = graph.node_weight(end_idx).unwrap();

            let distance = calculate_3d_distance(start_node, end_node);
            distances.push((start_idx, end_idx, distance));
        }
    }
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let distances = &mut distances[0..n_interblob_edges];
    distances.shuffle(rng);

    for connection in distances {
        graph.update_edge(
            connection.0, 
            connection.1, 
            EdgeData::with_color(connection.2, Color::TOMATO)
        );
    }

    return graph; 
}

fn connect_blobs(mut graph: Graph::<NodeData, EdgeData>, rng: &mut ChaCha8Rng)
    -> Graph::<NodeData, EdgeData> {

    //TODO: Arguments-to-be
    let n_candidates = 2;

    //Make sure each one is reached at least once in a full run
    //DANGEROUS: THE NODEDATA MIGHT BECOME OUTDATED
    let mut blob_order: Vec<(NodeIndex, NodeData)> = get_centers(&graph)
        .iter()
        .map(|x| (x.0, x.1.clone()))
        .collect();
    blob_order.shuffle(rng);

    let centre_indices: Vec<NodeIndex> = blob_order.iter().map(|x| x.0).collect();

    //TODO: TOO MANY EDGES
    //CORRECTION: IT SEEMS THAT WAY BECAUSE LINES GO THROUGH NODES
    //IT WORKS FINE WITH LOW n_candidates BUT I SHOULD FIX THIS
    for (start_idx, centre_node) in blob_order {
        println!("Connecting blob {:?}", start_idx);
        // let n_edges = graph.edges(start_idx).count();
        // dbg!(&n_edges);
        // if n_edges > start_node.n_connections {
        //     // dbg!("Triggered 1");
        //     continue;
        // }

        //TODO: CONTINUE FROM HERE, IDENTIFY OTHER CENTRES EITHER IN-LOOP 
        //OR FILTER OUT BEFOREHAND

        //Get vec from outer distances, filter down to other centres XXX
        //Use existing nodes in blob_order XXX

        //Regular candidate process but only with 3 candidates XXX
        //See if path exists between centres, continue if it does XXX
        //Otherwise, use function connect_blob XXX
        //First try to just connect centres directly to see how it looks XXX

        //Connect blob creates 2 vecs of (NodeIndex, outer_distance)
        //from each blob
        //Filters outer distance to only nodes in opposite side
        //Find X pairs of nodes in each blob with minimal distances
        //Just look at outer distances of one blob and find min distances
        //Connect

        //???
        //PROFIT

        //Same idea as with blob_order
        //Make sure it's sorted to get the closest nodes
        //Remove any non-center nodes
        let candidates = &centre_node.outer_distances;
        let candidates = get_sorted_distances(&candidates);
        let mut candidates: Vec<&(NodeIndex, f32)> = candidates
            .iter()
            .filter(|x| centre_indices.contains(&x.0))
            .collect();
        let candidates = &mut candidates[0..n_candidates];
        candidates.shuffle(rng);

        for (candidate_idx, _) in candidates {
            println!("Considering candidate {:?}", candidate_idx);

            //Holy shit, this just worked first try, wtf
            let path = algo::has_path_connecting(
                &graph,
                start_idx,
                *candidate_idx,
                None
            );
            if path == true {
                continue;
            }

            graph = connect_blob(graph, rng, start_idx, *candidate_idx);

            println!("Updated edge between blobs {:?} and {:?}", start_idx, candidate_idx);

            //It stops looking at candidates after one is updated to
            //keep it roughly uniformly distributed
            break;
        }

        //Might need to limit this to just centres to speed things up
        if is_blob_connected(&graph) == true {
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
    let graph = connect_blobs(graph, &mut rng);

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

    // dbg!(&graph);
    return graph;
}