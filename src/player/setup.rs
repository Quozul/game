use crate::camera::components::PlayerCameraBundle;
use crate::player::components::Player;
use crate::velocity::components::RigidBodyBundle;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn the player
    let rect_mesh = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0)));
    let player_id = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: rect_mesh,
                material: materials.add(Color::WHITE),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            },
            Player,
            RigidBodyBundle::default(),
        ))
        .id();

    // Spawn the camera
    commands.spawn(PlayerCameraBundle::new(player_id));
}
