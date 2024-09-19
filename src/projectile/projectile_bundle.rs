use crate::animation::components::AnimationConfig;
use crate::physics::colliders::circle_collider::CircleCollider;
use crate::physics::components::{DragCoefficient, RigidBodyBundle, Velocity};
use crate::projectile::components::{Damage, Lifetime, Projectile};
use crate::weapon::cannon::Cannon;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

#[derive(Bundle)]
pub struct ProjectileBundle {
    rigid_body: RigidBodyBundle,
    damage: Damage,
    life_time: Lifetime,
    circle_collider: CircleCollider,
    projectile: Projectile,
    sprite: SpriteBundle,
    texture: TextureAtlas,
    animation: AnimationConfig,
}

const PROJECTILE_SPEED: f32 = 1_000.0; // 1_000.0 seems like a good value
const PROJECTILE_MASS: f32 = 1.0;

impl ProjectileBundle {
    pub fn from_cannon(origin: &Transform, cannon: &Cannon, angle: Vec2) -> Self {
        let initial_velocity = angle * PROJECTILE_SPEED;
        let offset = origin.rotation.mul_vec3(cannon.offset);
        let animation_config_1 = AnimationConfig::repeating(0, 3, 10);

        Self {
            sprite: SpriteBundle {
                transform: Transform::from_translation(origin.translation + offset)
                    .with_rotation(Quat::from_rotation_z(angle.to_angle() - FRAC_PI_2)),
                texture: cannon.texture.clone(),
                ..default()
            },
            texture: TextureAtlas {
                layout: cannon.texture_atlas_layout.clone(),
                index: animation_config_1.first_sprite_index,
            },
            animation: animation_config_1,
            rigid_body: RigidBodyBundle::default()
                .with_mass(PROJECTILE_MASS)
                .with_initial_velocity(Velocity::linear(initial_velocity))
                .with_drag_coefficient(DragCoefficient::CIRCLE),
            damage: Damage(cannon.damage),
            projectile: Projectile,
            life_time: Lifetime::default(),
            circle_collider: CircleCollider::circle(1.0),
        }
    }
}
