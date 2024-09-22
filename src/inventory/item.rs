use crate::animation::components::AnimationConfig;
use crate::interact::bundle::InteractBundle;
use bevy::prelude::*;

#[derive(Component)]
pub struct Item<T>(pub T);

#[derive(Bundle)]
pub struct ItemBundle<T: Send + Sync + 'static> {
    item: Item<T>,
    animation_config: AnimationConfig,
    texture_atlas: TextureAtlas,
    sprite_bundle: SpriteBundle,
    interactive: InteractBundle,
}

impl<T: Send + Sync> ItemBundle<T> {
    pub fn new(
        item: T,
        sprite_bundle: SpriteBundle,
        texture_atlas: TextureAtlas,
        animation_config: AnimationConfig,
    ) -> Self {
        Self {
            // Texture
            sprite_bundle,
            texture_atlas,
            animation_config,
            // Physics
            interactive: InteractBundle::new(),
            // Item
            item: Item(item),
        }
    }
}
