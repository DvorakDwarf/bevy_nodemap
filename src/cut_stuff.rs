//Snippets of code I want to remember but are no longer used, safely ignore file

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

    // let light = PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 15000.0,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(0.0, 2.0, 1.0),
    //     ..default()
    // };
    // commands.spawn(light);