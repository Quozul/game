#![feature(let_chains)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod camera;
mod constants;
mod enemy;
mod map;
mod physics;
mod player;
mod projectile;
mod utils;

use crate::camera::post_processing::plugin::PostProcessPlugin;
use crate::enemy::plugin::EnemyPlugin;
use crate::map::plugin::MapPlugin;
use crate::physics::plugin::PhysicsPlugin;
use crate::physics::resources::PhysicsResource;
use crate::player::plugin::PlayerPlugin;
use crate::projectile::plugin::ProjectilePlugin;
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
        .insert_resource(PhysicsResource {
            air_density: 0.05,
            // The original value is 6.674*10E11 m3⋅kg−1⋅s−2
            // We adjusted it to 10E0 so that the smallest object that will be attracted is 1 unit in mass
            gravity: 6.674 * 10E0,
        })
        .add_plugins((
            DefaultPlugins.set(bevy::log::LogPlugin {
                level: bevy::log::Level::INFO,
                filter: "wgpu=error,naga=warn,game=trace".to_string(),
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
            MapPlugin,
            PlayerPlugin,
            EnemyPlugin,
            PhysicsPlugin,
            ProjectilePlugin,
            PostProcessPlugin,
            camera::plugin::CameraPlugin,
        ))
        .init_state::<AppState>()
        .add_systems(Update, switch_scene.run_if(in_state(AppState::Menu)))
        .run();
}

fn switch_scene(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame);
}
