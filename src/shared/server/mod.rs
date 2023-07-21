use std::time::Duration;

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy_quinnet::server::QuinnetServerPlugin;
use bevy_rapier2d::prelude::{
    NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode, Vect,
};

use crate::direction::handle_move;
use crate::health::{attack_enemies, despawn_dead};
use crate::server::message_events::{ClientConnectedEvent, ClientMoveEvent};
use crate::server::message_handlers::{
    handle_client_connected, handle_client_move, send_direction,
};
use crate::server::server::{
    handle_client_messages, handle_disconnected_clients, send_health, send_positions, spawn_slime,
    start_server,
};
use crate::server_entities::StaticServerEntity;
use crate::FIXED_TIMESTEP;

mod message_events;
mod message_handlers;
mod server;

pub fn start_server_app() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(
                FIXED_TIMESTEP,
            ))),
            QuinnetServerPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0),
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vect::ZERO,
            timestep_mode: TimestepMode::Fixed {
                dt: FIXED_TIMESTEP,
                substeps: 1,
            },
            ..default()
        })
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .insert_resource(StaticServerEntity::default())
        .add_event::<ClientConnectedEvent>()
        .add_event::<ClientMoveEvent>()
        .add_systems(PostStartup, start_server)
        .add_systems(
            Update,
            (
                handle_client_connected,
                handle_client_move,
                attack_enemies,
                despawn_dead,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                handle_disconnected_clients,
                handle_client_messages,
                send_positions,
                send_health,
                send_direction,
                spawn_slime,
                handle_move,
            ),
        )
        .run();
}
