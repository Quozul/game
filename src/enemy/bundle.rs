use crate::enemy::components::{Enemy, Health};
use crate::physics::colliders::circle_collider::CircleCollider;
use crate::physics::movements_components::RigidBodyBundle;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use std::f32::consts::PI;

#[derive(Bundle)]
pub struct EnemyBundle {
    material: MaterialMesh2dBundle<ColorMaterial>,
    collider: CircleCollider,
    rigid_body: RigidBodyBundle,
    health: Health,
    enemy: Enemy,
}

impl EnemyBundle {
    const DENSITY: f32 = 0.05;

    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        radius: f32,
        translation: Vec3,
    ) -> Self {
        let rect_mesh = Mesh2dHandle(meshes.add(Circle::new(radius)));
        let mass = PI * radius.powf(2.0) * EnemyBundle::DENSITY;
        let color = mass / 100.0;

        Self {
            material: MaterialMesh2dBundle {
                mesh: rect_mesh,
                material: materials.add(Color::linear_rgb(color, color, color)),
                transform: Transform::from_translation(translation),
                ..Default::default()
            },
            collider: CircleCollider::circle(radius),
            rigid_body: RigidBodyBundle::default().with_mass(mass),
            health: Health(radius as u32),
            enemy: Enemy,
        }
    }
}
