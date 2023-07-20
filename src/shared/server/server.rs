use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

use crate::direction::handle_move;
use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use bevy_quinnet::server::{QuinnetServerPlugin, Server, ServerConfiguration};
use bevy_rapier2d::prelude::{
    Collider, ExternalImpulse, NoUserData, RapierConfiguration, RapierPhysicsPlugin, RigidBody,
    TimestepMode, Vect,
};

use crate::health::{attack_enemies, Health};
use crate::messages::{ClientMessage, ServerMessage};
use crate::server::message_events::{ClientConnectedEvent, ClientMoveEvent};
use crate::server::message_handlers::{
    handle_client_connected, handle_client_move, send_direction,
};
use crate::server_entities::{NetworkServerEntity, StaticServerEntity};
use crate::FIXED_TIMESTEP;

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
        .add_systems(PostStartup, (start_server, spawn_floor))
        .add_systems(
            Update,
            (
                handle_client_connected,
                handle_client_move,
                handle_move,
                attack_enemies,
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
            ),
        )
        .run();
}

pub fn start_server(mut server: ResMut<Server>) {
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

pub fn spawn_floor(mut commands: Commands, mut server_entity_builder: ResMut<StaticServerEntity>) {
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(10.0, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(server_entity_builder.next())
        .insert(ExternalImpulse {
            impulse: Vec2::ZERO,
            torque_impulse: 0.0,
        })
        .insert(Health { health: 10 });
}

pub(crate) fn send_positions(
    mut server: ResMut<Server>,
    query: Query<(&NetworkServerEntity, &Transform), Changed<Transform>>,
) {
    if let Some(endpoint) = server.get_endpoint_mut() {
        for (server_entity, transform) in &query {
            endpoint.try_broadcast_message(ServerMessage::Position {
                id: server_entity.client_id,
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
                id: server_entity.client_id,
                new_health: health.health
            });
        }
    }
}

pub fn handle_client_messages(
    mut server: ResMut<Server>,
    mut client_connected_writer: EventWriter<ClientConnectedEvent>,
    mut client_move_writer: EventWriter<ClientMoveEvent>,
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
                    _ => {
                        println!("Received unknown message")
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
            if !endpoint.clients().contains(&network_entity.client_id) {
                commands.entity(entity).despawn();

                endpoint.try_broadcast_message(ServerMessage::Despawn {
                    id: network_entity.client_id,
                })
            }
        }
    }
}
