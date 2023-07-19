use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

use bevy::prelude::*;
use bevy_quinnet::client::certificate::CertificateVerificationMode;
use bevy_quinnet::client::connection::ConnectionConfiguration;
use bevy_quinnet::client::Client;

use shared::messages::{ClientMessage, ServerMessage};

use crate::camera_follow::FollowSubject;
use crate::message_handlers::spawn_player::SpawnPlayerEvent;
use crate::message_handlers::update_direction::UpdateDirection;
use crate::message_handlers::update_position::UpdatePositionEvent;
use crate::message_handlers::update_your_id::UpdateYourId;
use crate::AppState;

pub fn join_server(next_state: &mut NextState<AppState>, client: &mut Client, server_ip: &str) {
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
            .send_message(ClientMessage::Connected)
            .unwrap();
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

pub(crate) fn setup_in_game(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(OrthographicProjection {
            scale: 0.5,
            ..default()
        })
        .insert(FollowSubject);
}

pub(crate) fn handle_server_messages(
    mut client: ResMut<Client>,
    mut spawn_player_event_writer: EventWriter<SpawnPlayerEvent>,
    mut update_position_event_writer: EventWriter<UpdatePositionEvent>,
    mut update_your_id_event_writer: EventWriter<UpdateYourId>,
    mut update_direction_event_writer: EventWriter<UpdateDirection>,
) {
    if let Some(connection) = client.get_connection_mut() {
        while let Ok(Some(message)) = connection.receive_message::<ServerMessage>() {
            match message {
                // Match on your own message types ...
                ServerMessage::Spawn { id, x, y } => {
                    spawn_player_event_writer.send(SpawnPlayerEvent { id, x, y })
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
                ServerMessage::YourId { id } => {
                    update_your_id_event_writer.send(UpdateYourId { id });
                }
                ServerMessage::Direction { id, direction } => {
                    update_direction_event_writer.send(UpdateDirection { id, direction });
                }
                _ => {
                    println!("Received unknown message")
                }
            }
        }
    }
}
