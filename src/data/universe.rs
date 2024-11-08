use core::fmt::Debug;

use super::Blob;

//TODO: variant will be an array/vector/something with possible variants
//Which are picked randomly or weighted and then randomly picked
//There will also be some amount of nodes allocated per blob in a range
#[derive(Debug)]
pub struct Universe<B: Blob> {
    pub n_blobs: usize,
    //TODO: Not confident what this entails
    //Each blob variant is a different struct with trait Blob
    pub blob_variants: Vec<B>, 
    pub size: UniverseSize,
    pub blob_distance_tolerance: f32,
    pub n_blob_candidates: usize,
    pub min_connections: usize, //TODO: not actually used
    pub max_connections: usize, //TODO: not actually use
    pub n_sparse_nodes: usize,
    pub sparse_distance_tolerance: f32,
    pub n_sparse_connections: usize,
}

#[derive(Debug)]
pub struct UniverseSize {
    pub radius: f32,
    pub height: f32
}