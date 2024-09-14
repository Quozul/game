use crate::enemy::components::Enemy;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn one enemy
    let rect_mesh = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 50.0)));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: rect_mesh,
            material: materials.add(Color::linear_rgb(1.0, 0.5, 0.5)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Enemy,
    ));
}
