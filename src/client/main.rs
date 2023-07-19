#![feature(async_closure)]
#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy_quinnet::client::QuinnetClientPlugin;
use bevy_quinnet::shared::ClientId;
use bevy_rapier2d::prelude::{
    NoUserData, RapierConfiguration, RapierDebugRenderPlugin, RapierPhysicsPlugin, TimestepMode,
    Vect,
};
use leafwing_input_manager::prelude::InputManagerPlugin;

use shared::server::server::spawn_floor;
use shared::server_entities::StaticServerEntity;
use shared::FIXED_TIMESTEP;

use crate::animation::animate;
use crate::camera_follow::camera_follow;
use crate::client::{
    handle_server_messages, setup_in_game, start_client, start_server, text_input,
};
use crate::controls::{add_controller_to_self_player, jump, update_animation, Action};
use crate::menu::{setup_menu, MenuItem};
use crate::message_handlers::spawn_player::{handle_player_spawn, SpawnPlayerEvent};
use crate::message_handlers::update_direction::{handle_update_direction_event, UpdateDirection};
use crate::message_handlers::update_position::{handle_update_position_event, UpdatePositionEvent};
use crate::message_handlers::update_your_id::{handle_your_id_event, UpdateYourId};

mod animation;
mod camera_follow;
mod client;
mod controls;
mod menu;
pub mod message_handlers;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}

#[derive(Resource)]
pub struct MyId {
    id: ClientId,
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
        .add_event::<SpawnPlayerEvent>()
        .add_event::<UpdatePositionEvent>()
        .add_event::<UpdateYourId>()
        .add_event::<UpdateDirection>()
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
        .insert_resource(StaticServerEntity::default())
        .add_state::<AppState>()
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(OnExit(AppState::Menu), cleanup_menu)
        .add_systems(OnEnter(AppState::InGame), (setup_in_game, spawn_floor))
        .add_systems(
            Update,
            (
                start_client,
                start_server,
                camera_follow,
                text_input,
                add_controller_to_self_player,
                jump,
                update_animation,
                animate,
                handle_update_direction_event,
                handle_update_position_event,
                handle_your_id_event,
                handle_player_spawn,
            ),
        )
        .add_systems(FixedUpdate, handle_server_messages)
        .run();
}

fn cleanup_menu(mut commands: Commands, query: Query<(Entity, &MenuItem)>) {
    for (entity, _) in &query {
        commands.entity(entity).despawn_recursive()
    }
}
