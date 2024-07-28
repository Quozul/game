use std::time::Duration;

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy_quinnet::server::QuinnetServerPlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};

use crate::direction::handle_move;
use crate::gravity::get_rapier_configuration;
use crate::health::{animate_dead, attack_enemies, despawn_dead, slime_attack, tick_dead};
use crate::map::spawn_map;
use crate::server::message_events::{ClientConnectedEvent, ClientFacingEvent, ClientMoveEvent};
use crate::server::message_handlers::{
    handle_client_connected, handle_client_facing, handle_client_move, send_direction, send_facing,
};
use crate::server::server_events::{
    handle_client_messages, handle_disconnected_clients, send_health, send_positions, spawn_slime,
    start_server,
};
use crate::server_entities::StaticServerEntity;
use crate::FIXED_TIMESTEP;

mod message_events;
mod message_handlers;
mod server_events;

pub fn start_server_app() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(
                FIXED_TIMESTEP,
            ))),
            QuinnetServerPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0),
        ))
        .insert_resource(get_rapier_configuration())
        .insert_resource(Time::<Fixed>::from_seconds(FIXED_TIMESTEP as f64))
        .insert_resource(StaticServerEntity::default())
        .add_event::<ClientConnectedEvent>()
        .add_event::<ClientMoveEvent>()
        .add_event::<ClientFacingEvent>()
        .add_systems(PostStartup, (start_server, spawn_map))
        .add_systems(
            Update,
            (
                handle_client_connected,
                handle_client_move,
                handle_client_facing,
                attack_enemies,
                animate_dead,
                tick_dead,
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
                send_facing,
                spawn_slime,
                handle_move,
                slime_attack,
            ),
        )
        .run();
}
