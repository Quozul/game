use crate::animation::components::AnimationConfig;
use crate::inventory::item::ItemBundle;
use crate::weapon::cannon::Cannon;
use bevy::asset::{AssetServer, Assets};
use bevy::math::Vec3;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/Pickup Icon - Weapons - Auto Cannons.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 15, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_config_1 = AnimationConfig::repeating(0, 14, 10);

    let bullet_texture =
        asset_server.load("textures/Main ship weapon - Projectile - Auto cannon bullet.png");
    let bullet_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None);
    let bullet_texture_atlas_layout = texture_atlas_layouts.add(bullet_layout);

    // Spawn an item
    commands.spawn((
        ItemBundle::new(Cannon {
            texture: bullet_texture,
            offset: Vec3::new(0.0, 25.0, 1.0),
            texture_atlas_layout: bullet_texture_atlas_layout,
            recoil: 200.0,
            reload: 200,
            spread: 2.0,
            damage: 5,
        }),
        SpriteBundle {
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            texture: texture.clone(),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: animation_config_1.first_sprite_index,
        },
        animation_config_1,
    ));
}
