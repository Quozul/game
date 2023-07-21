use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};

pub fn spawn_map(mut commands: Commands) {
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(100.0, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 100.0, 0.0)));

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(100.0, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(10.0, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(100.0, 0.0, 0.0)));

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(10.0, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(-100.0, 0.0, 0.0)));
}
