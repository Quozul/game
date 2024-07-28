use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

use bevy::prelude::*;
use bevy_quinnet::client::certificate::CertificateVerificationMode;
use bevy_quinnet::client::connection::{
    ClientEndpointConfiguration, ConnectionEvent, ConnectionLostEvent,
};
use bevy_quinnet::client::QuinnetClient;
use bevy_quinnet::shared::channels::{ChannelType, ChannelsConfiguration};
use shared::messages::{ClientMessage, ServerMessage};
use shared::server_entities::NetworkServerEntity;

use crate::camera_follow::FollowSubject;
use crate::message_handlers::despawn_player::DespawnEntityEvent;
use crate::message_handlers::health_changed::HealthChangedEvent;
use crate::message_handlers::spawn_player::SpawnPlayerEvent;
use crate::message_handlers::spawn_slime::SpawnSlimeEvent;
use crate::message_handlers::update_direction::UpdateDirectionEvent;
use crate::message_handlers::update_facing::UpdateFacingEvent;
use crate::message_handlers::update_position::UpdatePositionEvent;
use crate::AppState;

pub(crate) fn join_server(
    next_state: &mut NextState<AppState>,
    client: &mut QuinnetClient,
    server_ip: &str,
) {
    if let Ok(ip) = IpAddr::from_str(server_ip) {
        debug!("Connecting...");

        ChannelsConfiguration::from_types(vec![ChannelType::OrderedReliable])
            .and_then(|channels_configuration| {
                client.open_connection(
                    ClientEndpointConfiguration::from_ips(
                        ip,
                        6000,
                        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                        0,
                    ),
                    CertificateVerificationMode::SkipVerification,
                    channels_configuration,
                )
            })
            .map(|_| {
                next_state.set(AppState::Connecting);
                client
                    .connection()
                    .try_send_message(ClientMessage::Connected);
            })
            .map_err(|err| error!("{err}"))
            .ok();
    }
}

pub(crate) fn on_connecting(
    mut next_state: ResMut<NextState<AppState>>,
    mut connection_event: EventReader<ConnectionEvent>,
) {
    for _ in connection_event.read() {
        info!("Client is connected");
        next_state.set(AppState::InGame);
    }
}

pub(crate) fn on_disconnected(
    mut next_state: ResMut<NextState<AppState>>,
    mut connection_event: EventReader<ConnectionLostEvent>,
) {
    for _ in connection_event.read() {
        info!("Client is disconnected");
        next_state.set(AppState::Menu);
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

pub(crate) fn close_connection(mut client: ResMut<QuinnetClient>) {
    let _ = client.close_all_connections();
}

pub(crate) fn clean_server_entities(
    mut commands: Commands,
    query: Query<Entity, Or<(With<NetworkServerEntity>, With<Camera>)>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub(crate) fn handle_server_messages(
    mut client: ResMut<QuinnetClient>,
    mut spawn_player_event_writer: EventWriter<SpawnPlayerEvent>,
    mut update_position_event_writer: EventWriter<UpdatePositionEvent>,
    mut update_direction_event_writer: EventWriter<UpdateDirectionEvent>,
    mut despawn_event_writer: EventWriter<DespawnEntityEvent>,
    mut health_changed_event_writer: EventWriter<HealthChangedEvent>,
    mut spawn_slime_event_writer: EventWriter<SpawnSlimeEvent>,
    mut update_facing_event_writer: EventWriter<UpdateFacingEvent>,
) {
    if let Some(connection) = client.get_connection_mut() {
        while let Ok(Some((_, message))) = connection.receive_message::<ServerMessage>() {
            match message {
                // Match on your own message types ...
                ServerMessage::SpawnPlayer { id, x, y, you } => {
                    spawn_player_event_writer.send(SpawnPlayerEvent { id, x, y, you });
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
                ServerMessage::Facing { id, facing } => {
                    update_facing_event_writer.send(UpdateFacingEvent { id, facing });
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
