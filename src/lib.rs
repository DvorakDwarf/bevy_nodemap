//THE container of the plugin

use bevy::{ecs::query::QueryData, prelude::*};
use bevy::color::palettes::css;
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

#[derive(Component, Debug)]
pub struct NodeId {
    pub id: NodeIndex
}

impl NodeId {
    fn from_id(id: NodeIndex) -> NodeId {
        return NodeId {
            id
        }
    }
}

impl<N: NodeData + 'static> Plugin for NodegraphPlugin<N> {
    fn build(&self, app: &mut App) {
        //build is only run once. GraphState is the real one we are changing
        let graph_state = GraphState::new(self.graph.clone());

        app
        // .add_plugins(BillboardPlugin) //Not using plugin until it's ported
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
            base_color: Color::Srgba(node.get_graph_data().color),
            reflectance: 0.02,
            unlit: false,
            ..default()
        };

        let node_transform = Transform::from_translation(node.get_vec());

        commands.spawn((
            Mesh3d(meshes.add(Mesh::from(Sphere::new(1.0)))),
            MeshMaterial3d(materials.add(node_material)),
            node_transform,
            NodeId::from_id(node_idx)
        ));
        

        //Create text underneath (explore options of crate)
        //Font used for text under nodes
        // let font_handle: Handle<Font> = asset_server.load("FiraSans-Regular.ttf");

        // TODO: BEVY_MOD_BILLBOARD NOT PORTED TO 0.15 YET
        // node_transform.translation.y += -1.25;
        // commands.spawn(BillboardTextBundle {
        //     transform: node_transform.with_scale(Vec3::splat(0.0085)),
        //     text: Text::from_section(
        //         format!("{}", node_idx.index()),
        //         TextSection {
        //             font: font_handle.clone(),
        //             font_size: 60.0,
        //             color: css::WHITE,
        //         },
        //     )
        //     .with_justify(JustifyText::Center),
        //     ..default()
        // });
    }
}

fn update_graph<N: NodeData + 'static>(
    global_state: Res<GraphState<N>>,
    query: Query<(&NodeId, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // global_state.graph.node_weights_mut().choose(&mut rand::thread_rng()).unwrap().get_mut_graph_data().color = css::LIME_GREEN;
    for (node_idx, material_handle) in query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            let node = global_state.graph.node_weight(node_idx.id).unwrap();
            if Color::Srgba(node.get_graph_data().color) != material.base_color {
                material.base_color = Color::Srgba(node.get_graph_data().color);
            }
        }
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
                
                gizmos.line(n1.get_vec(), n2.get_vec(), edge.color);
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
        color: Color::Srgba(css::WHITE),
        brightness: 700.0,
    });

}