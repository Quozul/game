use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

use bevy::prelude::*;
use bevy_quinnet::client::certificate::CertificateVerificationMode;
use bevy_quinnet::client::connection::ConnectionConfiguration;
use bevy_quinnet::client::Client;

use shared::messages::{ClientMessage, ServerMessage};

use crate::camera_follow::FollowSubject;
use crate::message_handlers::despawn_player::DespawnEntityEvent;
use crate::message_handlers::health_changed::HealthChangedEvent;
use crate::message_handlers::spawn_player::SpawnPlayerEvent;
use crate::message_handlers::spawn_slime::SpawnSlimeEvent;
use crate::message_handlers::update_direction::UpdateDirectionEvent;
use crate::message_handlers::update_position::UpdatePositionEvent;
use crate::AppState;

pub(crate) fn join_server(
    next_state: &mut NextState<AppState>,
    client: &mut Client,
    server_ip: &str,
) {
    if let Ok(ip) = IpAddr::from_str(server_ip) {
        debug!("Connecting...");

        client
            .open_connection(
                ConnectionConfiguration::from_ips(
                    ip,
                    6000,
                    IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                    0,
                ),
                CertificateVerificationMode::SkipVerification,
            )
            .unwrap();

        next_state.set(AppState::Connecting);

        client
            .connection()
            .try_send_message(ClientMessage::Connected);
    }
}

pub(crate) fn on_connecting(mut next_state: ResMut<NextState<AppState>>, client: ResMut<Client>) {
    if let Some(connection) = client.get_connection() {
        if connection.is_connected() {
            info!("Client is connected");
            next_state.set(AppState::InGame);
        }
    }
}

pub(crate) fn on_disconnected(mut next_state: ResMut<NextState<AppState>>, client: ResMut<Client>) {
    if let Some(connection) = client.get_connection() {
        if !connection.is_connected() {
            info!("Client is disconnected");
            next_state.set(AppState::Menu);
        }
    }
}

pub(crate) fn setup_in_game(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(OrthographicProjection {
            scale: 0.5,
            ..default()
        })
        .insert(FollowSubject);
}

pub(crate) fn close_connection(mut client: ResMut<Client>) {
    client.close_all_connections();
}

pub(crate) fn handle_server_messages(
    mut client: ResMut<Client>,
    mut spawn_player_event_writer: EventWriter<SpawnPlayerEvent>,
    mut update_position_event_writer: EventWriter<UpdatePositionEvent>,
    mut update_direction_event_writer: EventWriter<UpdateDirectionEvent>,
    mut despawn_event_writer: EventWriter<DespawnEntityEvent>,
    mut health_changed_event_writer: EventWriter<HealthChangedEvent>,
    mut spawn_slime_event_writer: EventWriter<SpawnSlimeEvent>,
) {
    if let Some(connection) = client.get_connection_mut() {
        while let Ok(Some(message)) = connection.receive_message::<ServerMessage>() {
            match message {
                // Match on your own message types ...
                ServerMessage::SpawnPlayer { id, x, y, you } => {
                    spawn_player_event_writer.send(SpawnPlayerEvent { id, x, y, you })
                }
                ServerMessage::Position {
                    id,
                    translation,
                    rotation,
                } => {
                    update_position_event_writer.send(UpdatePositionEvent {
                        id,
                        translation,
                        rotation,
                    });
                }
                ServerMessage::Direction { id, direction } => {
                    update_direction_event_writer.send(UpdateDirectionEvent { id, direction });
                }
                ServerMessage::Despawn { id } => {
                    despawn_event_writer.send(DespawnEntityEvent { id });
                }
                ServerMessage::Health { id, new_health } => {
                    health_changed_event_writer.send(HealthChangedEvent { id, new_health });
                }
                ServerMessage::SpawnSlime { id, x, y } => {
                    spawn_slime_event_writer.send(SpawnSlimeEvent { id, x, y });
                }
            }
        }
    }
}
