use bevy::math::Vec3;
use petgraph::graph::{NodeIndex, UnGraph};
use rand;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::blob_utils::{calculate_outer_distances, connect_blobs};
use crate::data::*;
use crate::sparse_nodes::add_sparse_nodes;

fn merge_graphs<N: NodeData + Clone>(
    graph1: &mut UnGraph<N, EdgeData>, 
    graph2: UnGraph<N, EdgeData>) {

    let index_offset = graph1.node_count();

    for node in graph2.node_weights() {
        let mut node = node.clone();

        node.get_mut_graph_data().neighbor_distances = node.get_graph_data().neighbor_distances.iter().map(|x| {
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

//TODO:
//Clean up generate_graph
pub fn generate_graph<N: NodeData + Clone> 
    (universe: Universe, dist: WeightedIndex<i32>) 
    -> UnGraph<N, EdgeData> {
    let mut rng = ChaCha8Rng::seed_from_u64(1337);
    let mut graph = UnGraph::<N, EdgeData>::new_undirected();
    //Make sure blobs don't spawn too close
    let mut locations: Vec<Location> = Vec::new();
    
    // //Add voids here. A location with no nodes but a distance tolerance
    // locations.push(Location{
    //     location_type: LocationType::Void,
    //     center_pos: Vec3::ZERO,
    //     distance_tolerance: 50.0
    // });

    //Place blobs
    for blob_idx in 0..universe.n_blobs {
        let selected_variant = &universe.blob_variants[dist.sample(&mut rng)];

        //TODO:
        //Kinda stupid, but a problem caused by trait
        let new_blob = match selected_variant{
            BlobWrapper::Disc(v) => {v.generate_blob(&universe, &mut locations, &mut rng, blob_idx)}
            BlobWrapper::Sphere(v) => {v.generate_blob(&universe, &mut locations, &mut rng, blob_idx)}
            BlobWrapper::Surface(v) => {v.generate_blob(&universe, &mut locations, &mut rng, blob_idx)}
        };
        merge_graphs(&mut graph, new_blob);
    }

    //Connect the blobs
    let graph = calculate_outer_distances(graph);
    let graph = connect_blobs(graph, &mut rng, &universe);
    let graph = add_sparse_nodes(graph, &mut rng, &universe);

    // dbg!(&graph);
    return graph;
}