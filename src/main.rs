use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_billboard::prelude::*;
use bevy_flycam::prelude::*;

mod data;
mod graph_gen;
mod disc_blob;
mod node_utils;
mod blob_utils;

use data::{BlobType, GlobalState, Universe, UniverseSize};
use graph_gen::generate_graph;

fn main() { 
    let universe = Universe {
        n_nodes: 20,
        n_blobs: 2,
        blob_variant: BlobType::Disc,
        size: UniverseSize {
            radius: 50.0,
            height: 20.0
        },
        no_no_distance: 1.0,
        blob_distance_tolerance: 30.0,
        n_blob_candidates: 1,
        n_member_candidates: 4,  
        fluff_requirement: 3.2,
        min_connections: 2, 
        max_connections: 6
    };
    let graph = generate_graph(universe);
    let global_state = GlobalState::new(graph);

    App::new()
        .insert_resource(global_state)
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PlayerPlugin)
        .add_plugins(BillboardPlugin)
        .add_systems(Startup, (
            // setup_billboard,
            spawn_graph,
            // spawn_cube,
            // spawn_camera,
            spawn_light
        ))
        .add_systems(Update, draw_lines)
        .run();
}

fn spawn_graph(
    mut commands: Commands,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    global_state: Res<GlobalState>,
    asset_server: Res<AssetServer>
) {
    //The state of the graph we want to display
    let graph = &global_state.graph;

    for node_idx in graph.node_indices() {
        let node = graph.node_weight(node_idx).unwrap();

        //How node will look like
        let node_material = StandardMaterial {
            base_color: node.color,
            reflectance: 0.02,
            unlit: false,
            ..default()
        };
        let node_handle = materials.add(node_material.clone());

        //Find where to put node
        let mut node_transform = Transform::from_translation(node.get_vec());

        //Create the 3D node
        let ball = PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere::new(1.0))),
            material: node_handle.clone(),
            transform: node_transform,
            ..default()
        };
        commands.spawn(ball);

        //Create text underneath (explore options of crate)
        //Font used for text under nodes
        let font_handle = asset_server.load("FiraSans-Regular.ttf");
        
        node_transform.translation.y += -1.25;
        commands.spawn(BillboardTextBundle {
            transform: node_transform.with_scale(Vec3::splat(0.0085)),
            text: Text::from_section(
                format!("{}", node_idx.index()),
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            )
            .with_justify(JustifyText::Center),
            ..default()
        });
    }
}

fn draw_lines(mut gizmos: Gizmos, global_state: Res<GlobalState>) {
    let graph = &global_state.graph;

    for edge_idx in graph.edge_indices() {
        let endpoints = graph.edge_endpoints(edge_idx);
        
        match endpoints {
            Some((n1, n2)) => {
                let edge = graph.edge_weight(edge_idx).unwrap();
                let n1 = graph.node_weight(n1).unwrap();
                let n2 = graph.node_weight(n2).unwrap();

                let (line, position) = Segment3d::from_points(n1.get_vec(), n2.get_vec());
                
                let edge_color = edge.color;

                gizmos.primitive_3d(
                    line,
                    position,
                    Quat::default(),
                    edge_color
                );
            },
            None => println!("Unexpected None edge"),
        }
    }
}

// fn spawn_cube(
//     mut commands: Commands,
//     mut meshes:ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>> 
// ) {
//     let default_material = StandardMaterial {
//         base_color: Color::rgb(0.8, 0.7, 0.6),
//         reflectance: 0.02,
//         unlit: false,
//         ..default()
//     };
//     let material_handle = materials.add(default_material.clone());


//     let cube = PbrBundle {
//         mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
//         material: material_handle,
//         transform: Transform::from_xyz(0.0, 0.0, 0.0),
//         ..default()
//     };

//     commands.spawn(cube);
// }

// fn spawn_camera(mut commands: Commands) {
//     let camera = Camera3dBundle {
//         transform: Transform::from_xyz(0.0, 0.0, 51.0),
//         ..default()
//     };

//     commands.spawn(camera);
// }

fn spawn_light(
    mut commands: Commands,
) {
    // let light = PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 15000.0,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(0.0, 2.0, 1.0),
    //     ..default()
    // };
    // commands.spawn(light);

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 700.0,
    });

}