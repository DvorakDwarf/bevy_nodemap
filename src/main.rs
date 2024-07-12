use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};
use petgraph::graph::Graph;

mod data;
use data::{EdgeData, GlobalState, NodeData};

//TODO
//Create resource with graph XXX
//Create spheres based on graph XXX
//Connect them with lines

fn main() {
    let mut graph = Graph::<NodeData, EdgeData>::new();
    let n1 = graph.add_node(NodeData::new(0.0, 0.0, 0.0));
    let n2 = graph.add_node(NodeData::new(5.0, 0.0, 0.0));
    let n3 = graph.add_node(NodeData::new(2.5, 2.5, 0.0));
    let n4 = graph.add_node(NodeData::new(10.0, 0.0, 0.0));

    graph.extend_with_edges(&[
        (n1, n2), (n2, n3), (n3, n1),
        (n2, n4)
    ]);

    dbg!(&graph);

    let mut global_state = GlobalState::new(graph);

    App::new()
        .insert_resource(global_state)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            spawn_graph,
            // spawn_cube,
            spawn_camera,
            spawn_light
        ))
        .run();
}

fn create_graph(mut commands: &mut Commands) {

}

fn spawn_graph(
    mut commands: Commands,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    global_state: Res<GlobalState>
) {
    let default_material = StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        reflectance: 0.02,
        unlit: false,
        ..default()
    };
    let material_handle = materials.add(default_material.clone());

    for node in global_state.graph.node_weights() {
        let ball = PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere::new(1.0))),
            material: material_handle.clone(),
            transform: Transform::from_xyz(node.x, node.y, node.z),
            ..default()
        };
        commands.spawn(ball);
    }
}

fn spawn_cube(
    mut commands: Commands,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>> 
) {
    let default_material = StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        reflectance: 0.02,
        unlit: false,
        ..default()
    };
    let material_handle = materials.add(default_material.clone());


    let cube = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
        material: material_handle,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    };

    commands.spawn(cube);
}

fn spawn_camera(mut commands: Commands) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 51.0),
        ..default()
    };

    commands.spawn(camera);
}

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
        color: Color::RED.into(),
        brightness: 1000.0,
    });

}