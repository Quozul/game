use crate::configuration::configuration_data::Configuration;
use crate::configuration::toml_loader::ConfigState;
use crate::constants::{MAP_SIZE, WALL_THICKNESS};
use crate::physics::colliders::polygon_collider::PolygonCollider;
use crate::physics::components::RigidBodyType;
use bevy::prelude::*;

pub fn setup_map(mut commands: Commands) {
    // Spawn walls
    commands.spawn((
        Transform::from_xyz(-(MAP_SIZE + WALL_THICKNESS) / 2.0, 0.0, 0.0),
        PolygonCollider::rectangle(WALL_THICKNESS, MAP_SIZE + WALL_THICKNESS * 2.0),
        RigidBodyType::Static,
    ));
    commands.spawn((
        Transform::from_xyz((MAP_SIZE + WALL_THICKNESS) / 2.0, 0.0, 0.0),
        PolygonCollider::rectangle(WALL_THICKNESS, MAP_SIZE + WALL_THICKNESS * 2.0),
        RigidBodyType::Static,
    ));
    commands.spawn((
        Transform::from_xyz(0.0, -(MAP_SIZE + WALL_THICKNESS) / 2.0, 0.0),
        PolygonCollider::rectangle(MAP_SIZE, WALL_THICKNESS),
        RigidBodyType::Static,
    ));
    commands.spawn((
        Transform::from_xyz(0.0, (MAP_SIZE + WALL_THICKNESS) / 2.0, 0.0),
        PolygonCollider::rectangle(MAP_SIZE, WALL_THICKNESS),
        RigidBodyType::Static,
    ));
}

pub fn setup_tutorial(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_section(
            "move: WASD\nshoot: left mouse button\npickup: E\ndrop: R\nzoom: mouse wheel",
            TextStyle {
                font_size: 16.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
    );
}

pub fn setup_items(
    mut state: ResMut<ConfigState<Configuration>>,
    custom_assets: Res<Assets<Configuration>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if state.used {
        return;
    }

    match custom_assets.get(&state.handle) {
        None => {
            info!("Custom Asset Not Ready");
        }
        Some(configuration) => {
            info!("{:?}", configuration);
            configuration
                .weapons
                .iter()
                .enumerate()
                .for_each(|(i, weapon)| {
                    commands.spawn(weapon.create_item_bundle(
                        &asset_server,
                        &mut texture_atlas_layouts,
                        Vec3::new(100.0 * (i + 1) as f32, 0.0, 0.0),
                    ));
                });
            state.used = true;
        }
    }
}
