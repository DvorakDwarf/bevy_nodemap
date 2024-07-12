use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Startup, setup)
        // .add_systems(Update, ui_example_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>) 
    {

    let cube_handle = meshes.add(Cuboid::new(4.0, 4.0, 4.0));
    let default_material = StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        reflectance: 0.02,
        unlit: true,
        ..default()
    };
    
    // The cube that will be rendered to the texture.
    commands.spawn(PbrBundle {
            mesh: cube_handle,
            material: default_material,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        })
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::SidePanel::right("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("This is a label");
    });
}

// bevy = "0.10.0"
// bevy_egui = "0.20.1"

// bevy = "0.13.2"
// bevy_egui = "0.27.0"
// egui = "0.27.2"