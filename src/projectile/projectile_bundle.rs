use crate::physics::colliders::circle_collider::CircleCollider;
use crate::physics::components::{DragCoefficient, RigidBodyBundle, Velocity};
use crate::projectile::components::{CannonProperties, Damage, Lifetime};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Bundle)]
pub struct ProjectileBundle {
    material: MaterialMesh2dBundle<ColorMaterial>,
    rigid_body: RigidBodyBundle,
    damage: Damage,
    life_time: Lifetime,
    circle_collider: CircleCollider,
}

const PROJECTILE_SPEED: f32 = 1_000.0; // 1_000.0 seems like a good value
const PROJECTILE_MASS: f32 = 1.0;

impl ProjectileBundle {
    pub fn from_cannon(origin: &Transform, cannon: &CannonProperties, angle: Vec2) -> Self {
        let bullet_mesh = Mesh2dHandle(cannon.mesh_handle.clone());
        let initial_velocity = angle * PROJECTILE_SPEED;

        let offset = origin.rotation.mul_vec3(cannon.offset);

        Self {
            material: MaterialMesh2dBundle {
                mesh: bullet_mesh,
                material: cannon.material_handle.clone(),
                transform: Transform::from_translation(origin.translation + offset),
                ..Default::default()
            },
            rigid_body: RigidBodyBundle::default()
                .with_mass(PROJECTILE_MASS)
                .with_initial_velocity(Velocity::linear(initial_velocity))
                .with_drag_coefficient(DragCoefficient::CIRCLE),
            damage: Damage(10),
            life_time: Lifetime::default(),
            circle_collider: CircleCollider::circle(1.0),
        }
    }
}
