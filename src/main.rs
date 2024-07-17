use rand::Rng;
use bevy::{core_pipeline::core_3d::graph, gizmos::gizmos, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_billboard::prelude::*;
use petgraph::graph::{Graph, NodeIndex};
use bevy_flycam::prelude::*;

mod data;
use data::{EdgeData, GlobalState, NodeData};

//TODO
//Create resource with graph XXX
//Create spheres based on graph XXX
//Connect them with lines

fn generate_graph(n_nodes: usize, n_neighbors: usize, distance: i32) -> Graph::<NodeData, EdgeData> {
    let mut graph = Graph::<NodeData, EdgeData>::new();

    let mut current_pos = Vec3::new(0.0, 0.0, 0.0);
    let mut node_data = NodeData::from(current_pos);
    node_data.color = Color::BLUE;

    let mut source = graph.add_node(node_data);

    let mut rng = rand::thread_rng();
    for _ in 0..n_nodes {
        for i in 0..rng.gen_range(1..=n_neighbors) {
            let mut neighbor_pos = current_pos.clone();
            neighbor_pos.x = (rng.gen_range(-distance..=distance)) as f32;
            neighbor_pos.y = (rng.gen_range(-distance..=distance)) as f32;
            neighbor_pos.z = (rng.gen_range(-distance..=distance)) as f32;

            let neighbor = graph.add_node(NodeData::from(neighbor_pos));
            graph.add_edge(source, neighbor, EdgeData::default());

            if i == n_neighbors-1 {
                source = neighbor;
                current_pos = neighbor_pos;
            }
        }        
    }

    return graph;
}

fn main() {
    // let mut graph = Graph::<NodeData, EdgeData>::new();
    // let n1 = graph.add_node(NodeData::new(0.0, 0.0, 0.0));
    // let n2 = graph.add_node(NodeData::new(5.0, 0.0, 0.0));
    // let n3 = graph.add_node(NodeData::new(2.5, 2.5, 0.0));
    // let n4 = graph.add_node(NodeData::new(10.0, 0.0, 0.0));

    // graph.extend_with_edges(&[
    //     (n1, n2), (n2, n3), (n3, n1),
    //     (n2, n4)
    // ]);

    // dbg!(&graph);

    let graph = generate_graph(30, 2, 10);
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

fn setup_billboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fira_sans_regular_handle = asset_server.load("FiraSans-Regular.ttf");
    commands.spawn(BillboardTextBundle {
        transform: Transform::from_scale(Vec3::splat(0.0085)),
        text: Text::from_sections([
            TextSection {
                value: "IMPORTANT".to_string(),
                style: TextStyle {
                    font_size: 60.0,
                    font: fira_sans_regular_handle.clone(),
                    color: Color::ORANGE,
                },
            },
            TextSection {
                value: " text".to_string(),
                style: TextStyle {
                    font_size: 60.0,
                    font: fira_sans_regular_handle.clone(),
                    color: Color::WHITE,
                },
            },
        ])
        .with_justify(JustifyText::Center),
        ..default()
    });
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

    //Font later used for text under nodes
    let font_handle = asset_server.load("FiraSans-Regular.ttf");

    for node in graph.node_weights() {
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
        node_transform.translation.y += -1.25;
        commands.spawn(BillboardTextBundle {
            transform: node_transform.with_scale(Vec3::splat(0.0085)),
            text: Text::from_section(
                format!("{} {} {}", node.x, node.y, node.z),
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

    for edge in graph.edge_indices() {
        let endpoints = graph.edge_endpoints(edge);
        
        match endpoints {
            Some((n1, n2)) => {
                let n1 = graph.node_weight(n1).unwrap();
                let n2 = graph.node_weight(n2).unwrap();

                let (line, position) = Segment3d::from_points(n1.get_vec(), n2.get_vec());
                
                gizmos.primitive_3d(
                    line,
                    position,
                    Quat::default(),
                    Color::RED
                );
            },
            None => println!("Unexpected None edge"),
        }
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
        mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
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
        color: Color::WHITE,
        brightness: 700.0,
    });

}