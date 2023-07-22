use bevy::prelude::*;
use bevy_quinnet::server::Server;

use crate::direction::{Facing, Move};
use crate::messages::ServerMessage;
use crate::player_bundle::PlayerBundle;
use crate::server::message_events::{ClientConnectedEvent, ClientFacingEvent, ClientMoveEvent};
use crate::server_entities::{NetworkServerEntity, StaticServerEntity};
use crate::slime_bundle::Slime;

pub(crate) fn handle_client_connected(
    mut server: ResMut<Server>,
    mut commands: Commands,
    mut static_server_entity: ResMut<StaticServerEntity>,
    mut client_connected_reader: EventReader<ClientConnectedEvent>,
    query: Query<(&NetworkServerEntity, &Transform), Without<Slime>>,
    slime_query: Query<(&NetworkServerEntity, &Transform), With<Slime>>,
) {
    if let Some(endpoint) = server.get_endpoint_mut() {
        for event in client_connected_reader.iter() {
            let id = static_server_entity.next_id();
            let x = 50.0;
            let y = 50.0;

            // Spawn the player
            commands.spawn(PlayerBundle::from_spawn_event(
                id,
                Some(event.client_id),
                x,
                y,
            ));

            // Send all players to the new player
            for (server_entity, transform) in &query {
                endpoint.try_send_message(
                    event.client_id,
                    ServerMessage::SpawnPlayer {
                        id: server_entity.id,
                        x: transform.translation.x,
                        y: transform.translation.y,
                        you: event.client_id == server_entity.id,
                    },
                );
            }

            // Send all slimes to the new player
            for (server_entity, transform) in &slime_query {
                endpoint.try_send_message(
                    event.client_id,
                    ServerMessage::SpawnSlime {
                        id: server_entity.id,
                        x: transform.translation.x,
                        y: transform.translation.y,
                    },
                );
            }

            // Send the new player to all players
            for client_id in endpoint.clients() {
                endpoint
                    .send_message(
                        client_id,
                        ServerMessage::SpawnPlayer {
                            id,
                            x,
                            y,
                            you: event.client_id == client_id,
                        },
                    )
                    .unwrap();
            }
        }
    }
}

pub(crate) fn handle_client_move(
    mut client_connected_reader: EventReader<ClientMoveEvent>,
    mut query: Query<(&NetworkServerEntity, &mut Move)>,
) {
    for event in client_connected_reader.iter() {
        for (server_entity, mut move_component) in &mut query {
            if server_entity.client_id == Some(event.client_id) {
                move_component.direction = event.direction;
                break;
            }
        }
    }
}

pub(crate) fn handle_client_facing(
    mut client_connected_reader: EventReader<ClientFacingEvent>,
    mut query: Query<(&NetworkServerEntity, &mut Facing)>,
) {
    for event in client_connected_reader.iter() {
        for (server_entity, mut facing) in &mut query {
            if server_entity.client_id == Some(event.client_id) {
                facing.angle = event.facing;
                break;
            }
        }
    }
}

pub(crate) fn send_direction(
    mut server: ResMut<Server>,
    mut query: Query<(&NetworkServerEntity, &Move), Changed<Move>>,
) {
    if let Some(endpoint) = server.get_endpoint_mut() {
        for (server_entity, move_component) in &mut query {
            endpoint.try_broadcast_message(ServerMessage::Direction {
                id: server_entity.id,
                direction: move_component.direction,
            });
        }
    }
}

pub(crate) fn send_facing(
    mut server: ResMut<Server>,
    mut query: Query<(&NetworkServerEntity, &Facing), Changed<Facing>>,
) {
    if let Some(endpoint) = server.get_endpoint_mut() {
        for (server_entity, facing) in &mut query {
            endpoint.try_broadcast_message(ServerMessage::Facing {
                id: server_entity.id,
                facing: facing.angle,
            });
        }
    }
}
