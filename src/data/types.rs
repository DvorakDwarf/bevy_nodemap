use core::fmt::Debug;

use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Center, Member, Sparse, Extension
}

#[derive(Debug)]
pub enum BlobType {
    Disc
}

#[derive(Debug)]
pub enum VoidType {
    Sphere
}

#[derive(Debug)]
pub enum LocationType {
    Blob(BlobType),
    Void(VoidType)
}

#[derive(Debug)]
pub struct Location {
    pub location_type: LocationType,
    pub center_pos: Vec3,
    pub distance_tolerance: f32
}