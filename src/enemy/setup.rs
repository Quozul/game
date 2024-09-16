use crate::enemy::components::Enemy;
use crate::physics::colliders::circle_collider::CircleCollider;
use crate::physics::movements_components::RigidBodyBundle;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn one enemy
    let rect_mesh = Mesh2dHandle(meshes.add(Circle::new(50.0)));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: rect_mesh,
            material: materials.add(Color::linear_rgb(1.0, 0.5, 0.5)),
            transform: Transform::from_xyz(500.0, 0.0, 0.0),
            ..Default::default()
        },
        CircleCollider::circle(50.0),
        RigidBodyBundle::default().with_mass(5.0),
        Enemy,
    ));

    // Spawn another enemy
    let rect_mesh = Mesh2dHandle(meshes.add(Circle::new(50.0)));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: rect_mesh,
            material: materials.add(Color::linear_rgb(1.0, 0.5, 0.5)),
            transform: Transform::from_xyz(500.0, 500.0, 0.0),
            ..Default::default()
        },
        CircleCollider::circle(50.0),
        RigidBodyBundle::default().with_mass(1.0),
        Enemy,
    ));
}
