//THE container of the plugin

use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;

mod data;
mod graph_gen;
mod blobs;
mod sparse_nodes;
mod node_utils;
mod blob_utils;
mod presets;

pub use data::{GraphState, NodeData, EdgeData, GenericNode, GraphData, NodeType};
use petgraph::prelude::*;

pub struct NodegraphPlugin<N: NodeData> {
    pub graph: UnGraph<N, EdgeData>
}

impl<N: NodeData> NodegraphPlugin<N> {
    pub fn default() -> NodegraphPlugin<N> {
        NodegraphPlugin { 
            graph: presets::preset_og() 
        }
    }

   pub fn from_graph(graph: UnGraph<N, EdgeData>) -> NodegraphPlugin<N> {
        NodegraphPlugin { 
            graph
        }
    }
}

impl<N: NodeData + 'static> Plugin for NodegraphPlugin<N> {
    fn build(&self, app: &mut App) {
        //build is only run once. GraphState is the real one we are changing
        let graph_state = GraphState::new(self.graph.clone());

        app
        .add_plugins(BillboardPlugin)
        .insert_resource(graph_state)
        .add_systems(Startup, (
            spawn_graph::<N>,
            spawn_light
        ))
        .add_systems(Update, (
            update_graph::<N>,
            draw_lines::<N>
        ));
    }
}

fn spawn_graph<N: NodeData + 'static>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    global_state: Res<GraphState<N>>,
    asset_server: Res<AssetServer>
) {
    //The state of the graph we want to display
    let graph = &global_state.graph;

    for node_idx in graph.node_indices() {
        let node = graph.node_weight(node_idx).unwrap();

        // if node.get_graph_data().role == NodeType::Member {
        //     dbg!(node.get_graph_data().color);   
        // }

        //How node will look like
        let node_material = StandardMaterial {
            base_color: node.get_graph_data().color,
            reflectance: 0.02,
            unlit: false,
            ..default()
        };
        let node_handle = materials.add(node_material);

        //Find where to put node
        let mut node_transform = Transform::from_translation(node.get_vec());

        //Create the 3D node
        let ball = PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere::new(1.0))),
            material: node_handle.clone(),
            transform: node_transform,
            ..default()
        };
        let ball_handle = commands.spawn(ball);

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

fn update_graph<N: NodeData + 'static>(mut meshes: ResMut<Assets<Mesh>>) {
    for mesh in meshes.iter() {
        dbg!(&mesh);
    }
}

fn draw_lines<N: NodeData + 'static >(mut gizmos: Gizmos, global_state: Res<GraphState<N>>) {
    let graph = &global_state.graph;

    //TODO:
    //Maybe check if any duplicate edges exist
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

fn spawn_light(
    mut commands: Commands,
) {
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 700.0,
    });

}