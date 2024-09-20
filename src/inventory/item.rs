use crate::animation::components::AnimationConfig;
use crate::physics::colliders::circle_collider::CircleCollider;
use crate::physics::components::Sensor;
use bevy::prelude::*;

#[derive(Component)]
pub struct Item<T>(pub T);

#[derive(Bundle)]
pub struct ItemBundle<T: Send + Sync + 'static> {
    collider: CircleCollider,
    sensor: Sensor,
    item: Item<T>,
    animation_config: AnimationConfig,
    atlas: TextureAtlas,
    bundle: SpriteBundle,
}

impl<T: Send + Sync> ItemBundle<T> {
    const PICKUP_RADIUS: f32 = 50.0;

    pub fn new(
        item: T,
        bundle: SpriteBundle,
        atlas: TextureAtlas,
        animation_config: AnimationConfig,
    ) -> Self {
        Self {
            // Texture
            bundle,
            atlas,
            animation_config,
            // Physics
            collider: CircleCollider::circle(Self::PICKUP_RADIUS),
            sensor: Sensor,
            // Item
            item: Item(item),
        }
    }
}
