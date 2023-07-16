use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

use crate::gravity::gravity;
use crate::FIXED_TIMESTEP;
use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use bevy_quinnet::server::{QuinnetServerPlugin, Server, ServerConfiguration};
use bevy_rapier2d::prelude::{
    Collider, KinematicCharacterController, NoUserData, RapierConfiguration, RapierPhysicsPlugin,
    RigidBody, TimestepMode, Vect,
};

use crate::messages::{ClientMessage, ServerMessage};
use crate::server_entities::{NetworkServerEntity, StaticServerEntity};

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
            gravity: Vect::new(0.0, -1.0),
            timestep_mode: TimestepMode::Fixed {
                dt: FIXED_TIMESTEP,
                substeps: 1,
            },
            ..default()
        })
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .insert_resource(StaticServerEntity::default())
        .add_systems(PostStartup, (start_server, spawn_floor))
        .add_systems(
            FixedUpdate,
            (handle_client_messages, send_positions, gravity),
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
        .insert(Collider::cuboid(500.0, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(server_entity_builder.next());
}

pub fn send_positions(
    mut server: ResMut<Server>,
    query: Query<(&NetworkServerEntity, &Transform)>,
) {
    if let Some(endpoint) = server.get_endpoint_mut() {
        for client_id in endpoint.clients() {
            for (server_entity, transform) in &query {
                endpoint
                    .send_message(
                        client_id,
                        ServerMessage::Position {
                            id: server_entity.client_id,
                            translation: transform.translation,
                            rotation: transform.rotation,
                        },
                    )
                    .unwrap();
            }
        }
    }
}

pub fn handle_client_messages(
    mut server: ResMut<Server>,
    mut commands: Commands,
    mut query: Query<(
        &NetworkServerEntity,
        &mut KinematicCharacterController,
        &Transform,
    )>,
) {
    if let Some(endpoint) = server.get_endpoint_mut() {
        for connected_client_id in endpoint.clients() {
            while let Some(message) =
                endpoint.try_receive_message_from::<ClientMessage>(connected_client_id)
            {
                match message {
                    ClientMessage::Connected => {
                        let x = 50.0;
                        let y = 50.0;

                        commands
                            .spawn(RigidBody::KinematicPositionBased)
                            .insert(Collider::cuboid(37.0 / 2.0, 37.0 / 2.0))
                            .insert(KinematicCharacterController::default())
                            .insert(TransformBundle::from(Transform::from_xyz(x, y, 0.0)))
                            .insert(NetworkServerEntity {
                                client_id: connected_client_id,
                            });

                        endpoint
                            .send_message(
                                connected_client_id,
                                ServerMessage::YourId {
                                    id: connected_client_id,
                                },
                            )
                            .unwrap();

                        // Send all players to the new player
                        for (server_entity, _, transform) in &mut query {
                            endpoint
                                .send_message(
                                    connected_client_id,
                                    ServerMessage::Spawn {
                                        id: server_entity.client_id,
                                        x: transform.translation.x,
                                        y: transform.translation.y,
                                    },
                                )
                                .unwrap();
                        }

                        // Send the new player to all players
                        for client_id in endpoint.clients() {
                            endpoint
                                .send_message(
                                    client_id,
                                    ServerMessage::Spawn {
                                        id: connected_client_id,
                                        x,
                                        y,
                                    },
                                )
                                .unwrap();
                        }
                    }
                    ClientMessage::Move { direction } => {
                        for (server_entity, mut controller, _) in &mut query {
                            if server_entity.client_id == connected_client_id {
                                let vel = direction.to_vec();
                                controller.translation = Some(vel);
                                break;
                            }
                        }
                    }
                    _ => {
                        println!("Received unknown message")
                    }
                }
            }
        }
    }
}
