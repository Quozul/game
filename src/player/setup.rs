use crate::camera::bundle::PlayerCameraBundle;
use crate::player::components::Player;
use crate::projectile::cannon_bundle::CannonBundle;
use crate::velocity::components::RigidBodyBundle;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn the player
    let bullet_texture = meshes.add(Ellipse::new(2.5, 2.5));
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
            CannonBundle::new(
                bullet_texture,
                Vec3::new(0.0, 0.0, 1.0),
                materials.add(Color::linear_rgb(0., 0., 1.)),
            ),
        ))
        .id();

    // Spawn the camera
    commands.spawn(PlayerCameraBundle::new(player_id));
}
