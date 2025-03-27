use crate::animation::components::AnimationConfig;
use crate::constants::{ASTEROID_COUNT, HALF_MAP};
use crate::enemy::bundle::EnemyBundle;
use crate::enemy::components::UiArrow;
use bevy::prelude::*;
use rand::Rng;

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut rng = rand::rng();

    let texture_handle = asset_server.load("textures/Asteroid 01 - Explode.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(96), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_config = AnimationConfig::once(0, 6, 10);

    let arrow_texture_handle: Handle<Image> = asset_server.load("textures/arrow.png");

    for _ in 0..ASTEROID_COUNT {
        let x = rng.random_range(-HALF_MAP..HALF_MAP);
        let y = rng.random_range(-HALF_MAP..HALF_MAP);
        let arrow = commands
            .spawn(SpriteBundle {
                transform: Transform::from_scale(Vec3::splat(0.5)),
                texture: arrow_texture_handle.clone(),
                ..default()
            })
            .id();

        commands.spawn((
            EnemyBundle::new(
                Vec3::new(x, y, 0.0),
                texture_handle.clone(),
                texture_atlas_layout.clone(),
                animation_config.clone(),
                rng.random_range(10..50),
            ),
            UiArrow(arrow),
        ));
    }
}
