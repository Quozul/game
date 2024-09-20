use crate::animation::components::AnimationConfig;
use crate::physics::colliders::circle_collider::CircleCollider;
use crate::physics::components::{DragCoefficient, RigidBodyBundle, Velocity};
use crate::projectile::components::{Damage, Lifetime, Projectile};
use crate::projectile::projectile_bundle::ProjectileBundle;
use bevy::asset::Handle;
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{default, Image, SpriteBundle, TextureAtlas, TextureAtlasLayout, Transform};
use std::f32::consts::FRAC_PI_2;

#[derive(Clone)]
pub struct Weapon {
    pub offset: Vec3,
    pub recoil: f32,
    pub reload: u64,
    pub spread: f32,
    pub projectile: Bullet,
    pub texture_handle: Handle<Image>,
    pub animation_config: AnimationConfig,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(Clone)]
pub struct Bullet {
    pub damage: u32,
    pub texture_handle: Handle<Image>,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
    pub speed: f32,
    pub mass: f32,
    pub radius: f32,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            damage: 1,
            texture_handle: Default::default(),
            texture_atlas_layout: Default::default(),
            speed: 1_000.0,
            mass: 1.0,
            radius: 1.0,
        }
    }
}

impl Bullet {
    pub fn create_projectile(&self, origin: Transform, angle: Vec2) -> ProjectileBundle {
        let initial_velocity = angle * self.speed;
        let animation_config = AnimationConfig::repeating(0, 3, 10, true);

        ProjectileBundle {
            // Texture
            sprite: SpriteBundle {
                transform: origin
                    .with_rotation(Quat::from_rotation_z(angle.to_angle() - FRAC_PI_2)),
                texture: self.texture_handle.clone(),
                ..default()
            },
            texture: TextureAtlas {
                layout: self.texture_atlas_layout.clone(),
                index: animation_config.first_sprite_index,
            },
            animation: animation_config,
            // Physics
            circle_collider: CircleCollider::circle(self.radius),
            rigid_body: RigidBodyBundle::default()
                .with_mass(self.mass)
                .with_initial_velocity(Velocity::linear(initial_velocity))
                .with_drag_coefficient(DragCoefficient::CIRCLE),
            // Projectile data
            projectile: Projectile,
            damage: Damage(self.damage),
            life_time: Lifetime::default(),
        }
    }
}
