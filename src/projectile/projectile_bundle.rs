use crate::physics::components::{Impulse, RigidBodyBundle};
use crate::projectile::components::{CannonProperties, Projectile};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Bundle)]
pub struct ProjectileBundle {
    material: MaterialMesh2dBundle<ColorMaterial>,
    rigid_body: RigidBodyBundle,
    projectile: Projectile,
    impulse: Impulse,
}

impl ProjectileBundle {
    pub fn from_cannon(origin: &Transform, cannon: &CannonProperties, angle: Vec2) -> Self {
        let bullet_mesh = Mesh2dHandle(cannon.mesh_handle.clone());
        let initial_velocity = angle * 1000.0;

        let offset = origin.rotation.mul_vec3(cannon.offset);

        Self {
            material: MaterialMesh2dBundle {
                mesh: bullet_mesh,
                material: cannon.material_handle.clone(),
                transform: Transform::from_translation(origin.translation + offset),
                ..Default::default()
            },
            rigid_body: RigidBodyBundle::new(1.0, initial_velocity, 0.05),
            projectile: Projectile,
            impulse: Impulse::default(),
        }
    }
}
