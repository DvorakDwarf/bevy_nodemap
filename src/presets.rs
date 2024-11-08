use petgraph::graph::UnGraph;
use rand::distributions::WeightedIndex;

use crate::data::{EdgeData, NodeData, Universe, UniverseSize};
use crate::blobs::{DiscBlob, SphereBlob, SphereSurfaceBlob};
use crate::graph_gen::generate_graph;

pub fn preset_og<N: NodeData>() -> UnGraph<N, EdgeData> {
    // Decent
    let disc_blob_1 = DiscBlob {
        n_nodes: 20,
        n_member_candidates: 4,
        fluff_requirement: 3.2,
        combo_chance: 20,
        no_no_distance: 5.0,
        radius: 25.0,
        height: 8.0,
        extension_radius: 10.0,
    };

    let universe = Universe {
        n_blobs: 10,
        blob_variants: vec![disc_blob_1],
        size: UniverseSize {
            radius: 100.0,
            height: 25.0
        },
        blob_distance_tolerance: 40.0,
        n_blob_candidates: 3,
        min_connections: 2, 
        max_connections: 6,
        n_sparse_nodes: 12,
        sparse_distance_tolerance: 12.0,
        n_sparse_connections: 3,
    };

    let weights = [1];
    let dist = WeightedIndex::new(&weights).unwrap();

    let graph = generate_graph(universe, dist);

    return graph;
}

pub fn preset_1() -> UnGraph<NodeData, EdgeData> {
    let disc_blob_1 = Box::new(DiscBlob {
        n_nodes: 20,
        n_member_candidates: 4,
        fluff_requirement: 3.2,
        combo_chance: 40,
        no_no_distance: 5.0,
        radius: 25.0,
        height: 8.0,
        extension_radius: 10.0,
    });
    let sphere_blob_1 = Box::new(SphereBlob {
        n_nodes: 20,
        n_member_candidates: 4,
        fluff_requirement: 3.2,
        combo_chance: 40,
        no_no_distance: 5.0,
        radius: 25.0,
        extension_radius: 10.0,
    });
    let sphere_surface_blob_1 = Box::new(SphereSurfaceBlob {
        n_nodes: 20,
        n_member_candidates: 3,
        fluff_requirement: 3.2,
        combo_chance: 40,
        no_no_distance: 7.0,
        radius: 20.0,
        extension_radius: 20.0,
    });

    let universe = Universe {
        n_blobs: 10,
        blob_variants: vec![disc_blob_1, sphere_blob_1, sphere_surface_blob_1],
        size: UniverseSize {
            radius: 120.0,
            height: 20.0
        },
        blob_distance_tolerance: 60.0,
        n_blob_candidates: 3,
        min_connections: 2, 
        max_connections: 6,
        n_sparse_nodes: 15,
        sparse_distance_tolerance: 12.0,
        n_sparse_connections: 3,
    };

    let weights = [1, 1, 1];
    let dist = WeightedIndex::new(&weights).unwrap();

    let graph = generate_graph(universe, dist);

    return graph;
}

pub fn preset_2() -> UnGraph<NodeData, EdgeData> {
    let disc_blob_1 = Box::new(DiscBlob {
        n_nodes: 20,
        n_member_candidates: 4,
        fluff_requirement: 3.2,
        combo_chance: 90,
        no_no_distance: 5.0,
        radius: 25.0,
        height: 8.0,
        extension_radius: 15.0,
    });
    let sphere_blob_1 = Box::new(SphereBlob {
        n_nodes: 20,
        n_member_candidates: 4,
        fluff_requirement: 3.2,
        combo_chance: 0,
        no_no_distance: 5.0,
        radius: 25.0,
        extension_radius: 10.0,
    });
    let sphere_surface_blob_1 = Box::new(SphereSurfaceBlob {
        n_nodes: 20,
        n_member_candidates: 3,
        fluff_requirement: 3.2,
        combo_chance: 0,
        no_no_distance: 7.0,
        radius: 20.0,
        extension_radius: 20.0,
    });

    let universe = Universe {
        n_blobs: 16,
        blob_variants: vec![disc_blob_1, sphere_blob_1, sphere_surface_blob_1],
        size: UniverseSize {
            radius: 200.0,
            height: 50.0
        },
        blob_distance_tolerance: 60.0,
        n_blob_candidates: 3,
        min_connections: 2, 
        max_connections: 6,
        n_sparse_nodes: 15,
        sparse_distance_tolerance: 12.0,
        n_sparse_connections: 3,
    };

    let weights = [80, 10, 10];
    let dist = WeightedIndex::new(&weights).unwrap();

    let graph = generate_graph(universe, dist);

    return graph;
}

pub fn preset_3() -> UnGraph<NodeData, EdgeData> {
    let disc_blob_1 = Box::new(DiscBlob {
        n_nodes: 16,
        n_member_candidates: 4,
        fluff_requirement: 3.2,
        combo_chance: 80,
        no_no_distance: 8.0,
        radius: 40.0,
        height: 8.0,
        extension_radius: 15.0,
    });
    let sphere_blob_1 = Box::new(SphereBlob {
        n_nodes: 20,
        n_member_candidates: 4,
        fluff_requirement: 3.2,
        combo_chance: 0,
        no_no_distance: 5.0,
        radius: 25.0,
        extension_radius: 10.0,
    });
    let sphere_surface_blob_1 = Box::new(SphereSurfaceBlob {
        n_nodes: 20,
        n_member_candidates: 3,
        fluff_requirement: 3.2,
        combo_chance: 0,
        no_no_distance: 7.0,
        radius: 20.0,
        extension_radius: 20.0,
    });

    let universe = Universe {
        n_blobs: 12,
        blob_variants: vec![disc_blob_1, sphere_blob_1, sphere_surface_blob_1],
        size: UniverseSize {
            radius: 200.0,
            height: 50.0
        },
        blob_distance_tolerance: 80.0,
        n_blob_candidates: 3,
        min_connections: 2, 
        max_connections: 6,
        n_sparse_nodes: 30,
        sparse_distance_tolerance: 12.0,
        n_sparse_connections: 3,
    };

    let weights = [80, 10, 10];
    let dist = WeightedIndex::new(&weights).unwrap();

    let graph = generate_graph(universe, dist);

    return graph;
}


pub fn preset_4() -> UnGraph<NodeData, EdgeData> {
    let disc_blob_1 = Box::new(DiscBlob {
        n_nodes: 16,
        n_member_candidates: 4,
        fluff_requirement: 3.2,
        combo_chance: 0,
        no_no_distance: 8.0,
        radius: 40.0,
        height: 8.0,
        extension_radius: 15.0,
    });
    let sphere_blob_1 = Box::new(SphereBlob {
        n_nodes: 20,
        n_member_candidates: 4,
        fluff_requirement: 3.2,
        combo_chance: 0,
        no_no_distance: 5.0,
        radius: 25.0,
        extension_radius: 10.0,
    });
    let sphere_surface_blob_1 = Box::new(SphereSurfaceBlob {
        n_nodes: 20,
        n_member_candidates: 3,
        fluff_requirement: 3.2,
        combo_chance: 0,
        no_no_distance: 7.0,
        radius: 20.0,
        extension_radius: 20.0,
    });

    let universe = Universe {
        n_blobs: 50,
        blob_variants: vec![disc_blob_1, sphere_blob_1, sphere_surface_blob_1],
        size: UniverseSize {
            radius: 200.0,
            height: 50.0
        },
        blob_distance_tolerance: 0.0,
        n_blob_candidates: 10,
        min_connections: 2, 
        max_connections: 6,
        n_sparse_nodes: 0,
        sparse_distance_tolerance: 12.0,
        n_sparse_connections: 3,
    };

    let weights = [80, 10, 10];
    let dist = WeightedIndex::new(&weights).unwrap();

    let graph = generate_graph(universe, dist);

    return graph;
}

//Big one
pub fn preset_5() -> UnGraph<NodeData, EdgeData> {
    let disc_blob_1 = Box::new(DiscBlob {
        n_nodes: 16,
        n_member_candidates: 4,
        fluff_requirement: 3.2,
        combo_chance: 0,
        no_no_distance: 8.0,
        radius: 40.0,
        height: 8.0,
        extension_radius: 15.0,
    });
    let sphere_blob_1 = Box::new(SphereBlob {
        n_nodes: 20,
        n_member_candidates: 4,
        fluff_requirement: 3.2,
        combo_chance: 0,
        no_no_distance: 5.0,
        radius: 25.0,
        extension_radius: 10.0,
    });
    let sphere_surface_blob_1 = Box::new(SphereSurfaceBlob {
        n_nodes: 20,
        n_member_candidates: 3,
        fluff_requirement: 3.2,
        combo_chance: 0,
        no_no_distance: 7.0,
        radius: 20.0,
        extension_radius: 20.0,
    });

    let universe = Universe {
        n_blobs: 80,
        blob_variants: vec![disc_blob_1, sphere_blob_1, sphere_surface_blob_1],
        size: UniverseSize {
            radius: 300.0,
            height: 80.0
        },
        blob_distance_tolerance: 0.0,
        n_blob_candidates: 10,
        min_connections: 2, 
        max_connections: 6,
        n_sparse_nodes: 0,
        sparse_distance_tolerance: 12.0,
        n_sparse_connections: 3,
    };

    let weights = [80, 10, 10];
    let dist = WeightedIndex::new(&weights).unwrap();

    let graph = generate_graph(universe, dist);

    return graph;
}

//Low number of big extending blobs
pub fn preset_6() -> UnGraph<NodeData, EdgeData> {
    let disc_blob_1 = Box::new(DiscBlob {
        n_nodes: 16,
        n_member_candidates: 4,
        fluff_requirement: 3.2,
        combo_chance: 70,
        no_no_distance: 8.0,
        radius: 40.0,
        height: 8.0,
        extension_radius: 15.0,
    });
    let sphere_blob_1 = Box::new(SphereBlob {
        n_nodes: 20,
        n_member_candidates: 4,
        fluff_requirement: 3.2,
        combo_chance: 0,
        no_no_distance: 5.0,
        radius: 25.0,
        extension_radius: 10.0,
    });
    let sphere_surface_blob_1 = Box::new(SphereSurfaceBlob {
        n_nodes: 20,
        n_member_candidates: 3,
        fluff_requirement: 3.2,
        combo_chance: 0,
        no_no_distance: 7.0,
        radius: 20.0,
        extension_radius: 20.0,
    });

    let universe = Universe {
        n_blobs: 30,
        blob_variants: vec![disc_blob_1, sphere_blob_1, sphere_surface_blob_1],
        size: UniverseSize {
            radius: 300.0,
            height: 80.0
        },
        blob_distance_tolerance: 0.0,
        n_blob_candidates: 10,
        min_connections: 2, 
        max_connections: 6,
        n_sparse_nodes: 0,
        sparse_distance_tolerance: 12.0,
        n_sparse_connections: 3,
    };

    let weights = [80, 10, 10];
    let dist = WeightedIndex::new(&weights).unwrap();

    let graph = generate_graph(universe, dist);

    return graph;
}