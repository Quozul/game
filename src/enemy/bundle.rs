use crate::enemy::components::{Enemy, Health};
use crate::physics::colliders::polygon_collider::PolygonCollider;
use crate::physics::components::RigidBodyBundle;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use std::f32::consts::PI;

#[derive(Bundle)]
pub struct EnemyBundle {
    material: MaterialMesh2dBundle<ColorMaterial>,
    collider: PolygonCollider,
    rigid_body: RigidBodyBundle,
    health: Health,
    enemy: Enemy,
}

impl EnemyBundle {
    const DENSITY: f32 = 0.05;

    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        circumradius: f32,
        sides: usize,
        translation: Vec3,
        linear_velocity: Vec2,
    ) -> Self {
        let mesh = Mesh2dHandle(meshes.add(RegularPolygon::new(circumradius, sides)));
        let mass = PI * circumradius.powf(2.0) * EnemyBundle::DENSITY;
        let color = mass / 100.0;

        Self {
            material: MaterialMesh2dBundle {
                mesh,
                material: materials.add(Color::linear_rgb(color, color, color)),
                transform: Transform::from_translation(translation),
                ..Default::default()
            },
            collider: PolygonCollider::regular_polygon(circumradius, sides),
            rigid_body: RigidBodyBundle::default()
                .with_mass(mass)
                .with_initial_velocity(linear_velocity),
            health: Health(circumradius as u32),
            enemy: Enemy,
        }
    }
}
