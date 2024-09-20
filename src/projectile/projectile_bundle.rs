use crate::animation::components::AnimationConfig;
use crate::physics::colliders::circle_collider::CircleCollider;
use crate::physics::components::RigidBodyBundle;
use crate::projectile::components::{Damage, Lifetime, Projectile};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub rigid_body: RigidBodyBundle,
    pub damage: Damage,
    pub life_time: Lifetime,
    pub circle_collider: CircleCollider,
    pub projectile: Projectile,
    pub sprite: SpriteBundle,
    pub texture: TextureAtlas,
    pub animation: AnimationConfig,
}
