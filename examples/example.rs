use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_flycam::prelude::*;

use bevy_nodemap::{NodegraphPlugin, GenericNode};

fn main() { 
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00012, // default: 0.00012
            speed: 36.0, // default: 12.0
        })
        .add_plugins(NodegraphPlugin::<GenericNode>::default())
        .run();
}