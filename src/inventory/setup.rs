use crate::animation::components::AnimationConfig;
use crate::inventory::item::ItemBundle;
use crate::weapon::weapon::{Bullet, Weapon};
use bevy::asset::{AssetServer, Assets};
use bevy::math::Vec3;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let item_texture = asset_server.load("textures/Pickup Icon - Weapons - Auto Cannons.png");
    let item_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 15, 1, None, None);
    let item_texture_atlas_layout = texture_atlas_layouts.add(item_layout);
    let item_animation_config = AnimationConfig::repeating(0, 14, 10, true);

    let bullet_texture =
        asset_server.load("textures/Main ship weapon - Projectile - Auto cannon bullet.png");
    let bullet_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None);
    let bullet_texture_atlas_layout = texture_atlas_layouts.add(bullet_layout);

    let weapon_texture = asset_server.load("textures/Main Ship - Weapons - Auto Cannon.png");
    let cannon_layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 7, 1, None, None);
    let weapon_texture_atlas_layout = texture_atlas_layouts.add(cannon_layout);
    let weapon_animation_config = AnimationConfig::repeating(0, 6, 10, false);

    // Spawn an item
    commands.spawn((
        ItemBundle::new(Weapon {
            texture_handle: weapon_texture,
            texture_atlas_layout: weapon_texture_atlas_layout,
            animation_config: weapon_animation_config,
            offset: Vec3::new(0.0, 25.0, 1.0),
            recoil: 200.0,
            reload: 200,
            spread: 2.0,
            projectile: Bullet {
                texture_handle: bullet_texture,
                texture_atlas_layout: bullet_texture_atlas_layout,
                damage: 5,
                radius: 5.0,
                ..default()
            },
        }),
        SpriteBundle {
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            texture: item_texture.clone(),
            ..default()
        },
        TextureAtlas {
            layout: item_texture_atlas_layout.clone(),
            index: item_animation_config.first_sprite_index,
        },
        item_animation_config,
    ));
}
