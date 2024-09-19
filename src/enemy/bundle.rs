use crate::animation::components::AnimationConfig;
use crate::enemy::components::{Enemy, Health};
use crate::physics::colliders::circle_collider::CircleCollider;
use crate::physics::components::{RigidBodyBundle, Velocity};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct EnemyBundle {
    collider: CircleCollider,
    rigid_body: RigidBodyBundle,
    health: Health,
    enemy: Enemy,
    sprite: SpriteBundle,
    texture: TextureAtlas,
    animation: AnimationConfig,
}

impl EnemyBundle {
    pub fn new(
        translation: Vec3,
        initial_velocity: Vec2,
        texture_handle: Handle<Image>,
        texture_atlas_layout: Handle<TextureAtlasLayout>,
        animation_config: AnimationConfig,
        health: u32,
    ) -> Self {
        Self {
            // Texture and animation
            sprite: SpriteBundle {
                transform: Transform::from_translation(translation),
                texture: texture_handle.clone(),
                ..default()
            },
            texture: TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config.first_sprite_index,
            },
            animation: animation_config,
            // Physics
            collider: CircleCollider::circle(24.0),
            rigid_body: RigidBodyBundle::default()
                .with_mass(24.0)
                .with_initial_velocity(Velocity::linear(initial_velocity)),
            // Enemy
            health: Health(health),
            enemy: Enemy,
        }
    }
}
