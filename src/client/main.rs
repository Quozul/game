#![feature(async_closure)]
#![feature(let_chains)]
#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_quinnet::client::QuinnetClientPlugin;
use bevy_rapier2d::prelude::{
    NoUserData, RapierConfiguration, RapierDebugRenderPlugin, RapierPhysicsPlugin, TimestepMode,
    Vect,
};
use leafwing_input_manager::prelude::InputManagerPlugin;

use shared::direction::handle_move;
use shared::map::spawn_map;
use shared::FIXED_TIMESTEP;

use crate::animations::entity_animations::{flip_animation, update_facing_direction};
use crate::camera_follow::camera_follow;
use crate::client::{
    clean_server_entities, close_connection, handle_server_messages, on_connecting,
    on_disconnected, setup_in_game,
};
use crate::controls::{add_controller_to_self_player, attack, controls, mouse_controls, Action};
use crate::display_health::display_health;
use crate::menu::{display_network_stats, setup_assets, ui_example_system, AssetsLoading, UiState};
use crate::message_handlers::despawn_player::{handle_entity_despawn, DespawnEntityEvent};
use crate::message_handlers::health_changed::{handle_health_change, HealthChangedEvent};
use crate::message_handlers::spawn_player::{handle_player_spawn, SpawnPlayerEvent};
use crate::message_handlers::spawn_slime::{handle_slime_spawn, SpawnSlimeEvent};
use crate::message_handlers::update_direction::{
    handle_update_direction_event, UpdateDirectionEvent,
};
use crate::message_handlers::update_facing::{handle_update_facing_event, UpdateFacingEvent};
use crate::message_handlers::update_position::{handle_update_position_event, UpdatePositionEvent};
use animations::animate::animate;
use animations::entity_animations::{update_player_animation, update_slime_animation};

mod animations;
mod camera_follow;
mod client;
mod controls;
mod display_health;
mod menu;
pub mod message_handlers;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub(crate) enum AppState {
    #[default]
    Menu,
    Connecting,
    InGame,
}

#[derive(Resource)]
pub(crate) struct MyId {
    id: u64,
    entity: Option<Entity>,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            QuinnetClientPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(InputManagerPlugin::<Action>::default())
        .add_plugins(EguiPlugin)
        .add_event::<SpawnPlayerEvent>()
        .add_event::<UpdatePositionEvent>()
        .add_event::<UpdateDirectionEvent>()
        .add_event::<DespawnEntityEvent>()
        .add_event::<HealthChangedEvent>()
        .add_event::<SpawnSlimeEvent>()
        .add_event::<UpdateFacingEvent>()
        .init_resource::<UiState>()
        .insert_resource(AssetsLoading {
            slime: None,
            player: None,
            font: None,
        })
        .insert_resource(RapierConfiguration {
            gravity: Vect::ZERO,
            timestep_mode: TimestepMode::Fixed {
                dt: FIXED_TIMESTEP,
                substeps: 1,
            },
            ..default()
        })
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .insert_resource(MyId {
            id: 0,
            entity: None,
        })
        .add_state::<AppState>()
        .add_systems(Startup, setup_assets)
        .add_systems(OnEnter(AppState::InGame), (setup_in_game, spawn_map))
        .add_systems(
            OnExit(AppState::InGame),
            (close_connection, clean_server_entities),
        )
        .add_systems(Update, ui_example_system.run_if(in_state(AppState::Menu)))
        .add_systems(Update, on_connecting.run_if(in_state(AppState::Connecting)))
        .add_systems(Update, on_disconnected.run_if(in_state(AppState::InGame)))
        .add_systems(
            Update,
            (
                camera_follow,
                add_controller_to_self_player,
                attack,
                controls,
                mouse_controls,
                flip_animation,
                update_facing_direction,
                update_player_animation,
                update_slime_animation,
                handle_update_facing_event,
                animate,
                handle_player_spawn,
                handle_update_direction_event,
                handle_update_position_event,
                handle_entity_despawn,
                handle_health_change,
                handle_slime_spawn,
                display_network_stats,
                display_health,
                handle_move,
            ),
        )
        .add_systems(FixedUpdate, handle_server_messages)
        .run();
}
