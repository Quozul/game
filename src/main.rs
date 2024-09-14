#![feature(let_chains)]

mod camera;
mod enemy;
mod player;
mod projectile;
mod utils;
mod velocity;

use crate::enemy::plugin::EnemyPlugin;
use crate::player::plugin::PlayerPlugin;
use crate::projectile::plugin::ProjectilePlugin;
use crate::velocity::plugin::VelocityPlugin;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(bevy::log::LogPlugin {
                level: bevy::log::Level::INFO,
                filter: "wgpu=error,naga=warn,game=debug".to_string(),
                ..default()
            }),
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.0, 1.0, 0.0),
                        font: default(),
                    },
                },
            },
        ))
        .add_plugins((
            PlayerPlugin,
            EnemyPlugin,
            VelocityPlugin,
            ProjectilePlugin,
            camera::plugin::CameraPlugin,
        ))
        .init_state::<AppState>()
        .add_systems(Update, switch_scene.run_if(in_state(AppState::Menu)))
        .run();
}

fn switch_scene(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame);
}
