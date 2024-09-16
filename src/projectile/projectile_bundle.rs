use crate::physics::movements_components::{DragCoefficient, RigidBodyBundle};
use crate::projectile::components::{CannonProperties, Lifetime, Projectile};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Bundle)]
pub struct ProjectileBundle {
    material: MaterialMesh2dBundle<ColorMaterial>,
    rigid_body: RigidBodyBundle,
    projectile: Projectile,
    life_time: Lifetime,
}

const PROJECTILE_SPEED: f32 = 1_000.0; // 1_000.0 seems like a good value

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
                .with_mass(1.0)
                .with_initial_velocity(initial_velocity)
                .with_drag_coefficient(DragCoefficient::CIRCLE),
            projectile: Projectile,
            life_time: Lifetime::default(),
        }
    }
}
