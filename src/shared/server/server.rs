use std::net::{IpAddr, Ipv4Addr};

use bevy::prelude::*;
use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use bevy_quinnet::server::{Server, ServerConfiguration};
use rand::{thread_rng, Rng};

use crate::health::Health;
use crate::messages::{ClientMessage, ServerMessage};
use crate::server::message_events::{ClientConnectedEvent, ClientFacingEvent, ClientMoveEvent};
use crate::server_entities::{NetworkServerEntity, StaticServerEntity};
use crate::slime_bundle::{Slime, SlimeBundle};

pub(crate) fn start_server(mut server: ResMut<Server>) {
    server
        .start_endpoint(
            ServerConfiguration::from_ip(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 6000),
            CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: String::from("localhost"),
            },
        )
        .unwrap();

    println!("Server started");
}

pub(crate) fn send_positions(
    mut server: ResMut<Server>,
    query: Query<(&NetworkServerEntity, &Transform), Changed<Transform>>,
) {
    if let Some(endpoint) = server.get_endpoint_mut() {
        for (server_entity, transform) in &query {
            endpoint.try_broadcast_message(ServerMessage::Position {
                id: server_entity.id,
                translation: transform.translation,
                rotation: transform.rotation,
            });
        }
    }
}

pub(crate) fn send_health(
    mut server: ResMut<Server>,
    query: Query<(&NetworkServerEntity, &Health), Changed<Health>>,
) {
    if let Some(endpoint) = server.get_endpoint_mut() {
        for (server_entity, health) in &query {
            endpoint.try_broadcast_message(ServerMessage::Health {
                id: server_entity.id,
                new_health: health.health,
            });
        }
    }
}

pub(crate) fn handle_client_messages(
    mut server: ResMut<Server>,
    mut client_connected_writer: EventWriter<ClientConnectedEvent>,
    mut client_move_writer: EventWriter<ClientMoveEvent>,
    mut client_facing_writer: EventWriter<ClientFacingEvent>,
) {
    if let Some(endpoint) = server.get_endpoint_mut() {
        for client_id in endpoint.clients() {
            while let Some(message) = endpoint.try_receive_message_from::<ClientMessage>(client_id)
            {
                match message {
                    ClientMessage::Connected => {
                        client_connected_writer.send(ClientConnectedEvent { client_id });
                    }
                    ClientMessage::Move { direction } => {
                        client_move_writer.send(ClientMoveEvent {
                            client_id,
                            direction,
                        });
                    }
                    ClientMessage::Facing { facing } => {
                        client_facing_writer.send(ClientFacingEvent { client_id, facing });
                    }
                }
            }
        }
    }
}

pub(crate) fn handle_disconnected_clients(
    mut commands: Commands,
    mut server: ResMut<Server>,
    query: Query<(Entity, &NetworkServerEntity)>,
) {
    if let Some(endpoint) = server.get_endpoint_mut() {
        for (entity, network_entity) in &query {
            if let Some(client_id) = network_entity.client_id {
                if !endpoint.clients().contains(&client_id) {
                    commands.entity(entity).despawn();

                    endpoint.try_broadcast_message(ServerMessage::Despawn {
                        id: network_entity.id,
                    })
                }
            }
        }
    }
}

pub(crate) fn spawn_slime(
    mut commands: Commands,
    mut static_server_entity: ResMut<StaticServerEntity>,
    mut server: ResMut<Server>,
    query: Query<&Slime>,
) {
    if query.iter().count() < 3 {
        let id = static_server_entity.next_id();
        let mut rng = thread_rng();
        let x = rng.gen_range(-50.0..50.0);
        let y = rng.gen_range(-50.0..50.0);

        commands.spawn(SlimeBundle::from_spawn_event(id, x, y));

        if let Some(endpoint) = server.get_endpoint_mut() {
            endpoint.try_broadcast_message(ServerMessage::SpawnSlime { id, x, y });
        }
    }
}
