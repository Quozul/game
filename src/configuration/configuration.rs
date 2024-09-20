use crate::animation::components::AnimationConfig;
use crate::inventory::item::ItemBundle;
use crate::weapon::weapon::{Bullet, Weapon};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::math::{UVec2, Vec3};
use bevy::prelude::*;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize, Default, Clone)]
struct AnimatedTexture {
    texture_path: String,
    tile_size: u32,
    columns: u32,
}

impl AnimatedTexture {
    pub fn load(
        &self,
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> (Handle<Image>, Handle<TextureAtlasLayout>, AnimationConfig) {
        let texture = asset_server.load(self.texture_path.clone());
        let layout = TextureAtlasLayout::from_grid(
            UVec2::splat(self.tile_size),
            self.columns,
            1,
            None,
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_config = AnimationConfig::repeating(0, (self.columns - 1) as usize, 10, true);

        (texture, texture_atlas_layout, animation_config)
    }
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct WeaponConfig {
    pickup_icon: AnimatedTexture,
    projectile: AnimatedTexture,
    weapon: AnimatedTexture,
    offset: Vec3,
    recoil: f32,
    reload: f32,
    spread: f32,
    damage: u32,
    radius: f32,
}

impl WeaponConfig {
    pub fn create_item_bundle(
        &self,
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
        translation: Vec3,
    ) -> ItemBundle<Weapon> {
        let (weapon_texture, weapon_texture_atlas_layout, weapon_animation_config) =
            self.weapon.load(asset_server, texture_atlas_layouts);
        let (bullet_texture, bullet_texture_atlas_layout, bullet_animation_config) =
            self.projectile.load(asset_server, texture_atlas_layouts);
        let (item_texture, item_texture_atlas_layout, item_animation_config) =
            self.pickup_icon.load(asset_server, texture_atlas_layouts);

        ItemBundle::new(
            Weapon {
                config: self.clone(),
                texture_handle: weapon_texture,
                texture_atlas_layout: weapon_texture_atlas_layout,
                animation_config: weapon_animation_config,
                offset: self.offset,
                recoil: self.recoil,
                reload: Duration::from_secs_f32(self.reload),
                spread: self.spread,
                projectile: Bullet {
                    texture_handle: bullet_texture,
                    texture_atlas_layout: bullet_texture_atlas_layout,
                    animation_config: bullet_animation_config,
                    damage: self.damage,
                    radius: self.radius,
                    ..default()
                },
            },
            SpriteBundle {
                transform: Transform::from_translation(translation),
                texture: item_texture.clone(),
                ..default()
            },
            TextureAtlas {
                layout: item_texture_atlas_layout.clone(),
                index: item_animation_config.first_sprite_index,
            },
            item_animation_config,
        )
    }
}

#[derive(Asset, TypePath, Debug, Deserialize, Default, Clone)]
pub struct Configuration {
    pub weapons: Vec<WeaponConfig>,
}
