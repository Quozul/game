use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy_quinnet::server::certificate::CertificateRetrievalMode;
use bevy_quinnet::server::{QuinnetServerPlugin, Server, ServerConfiguration};
use bevy_rapier2d::prelude::{
    Ccd, Collider, GravityScale, KinematicCharacterController, NoUserData, RapierConfiguration,
    RapierPhysicsPlugin, RigidBody, Sleeping, Vect, Velocity,
};

use crate::messages::{ClientMessage, ServerMessage};
use crate::server_entities::ServerEntityId;

pub fn start_server_app() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 20.0,
            ))),
            QuinnetServerPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0),
        ))
        .insert_resource(ServerEntityId::default())
        .insert_resource(RapierConfiguration {
            gravity: Vect::new(0.0, 0.0),
            ..default()
        })
        .add_systems(PostStartup, start_server)
        .add_systems(Update, (handle_client_messages, send_positions))
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

#[derive(Component)]
pub struct ServerEntity {
    client_id: u64,
}

pub fn send_positions(mut server: ResMut<Server>, query: Query<(&ServerEntity, &Transform)>) {
    if let Some(endpoint) = server.get_endpoint_mut() {
        for client_id in endpoint.clients() {
            for (server_entity, transform) in &query {
                endpoint
                    .send_message(
                        client_id,
                        ServerMessage::Position {
                            id: server_entity.client_id,
                            x: transform.translation.x,
                            y: transform.translation.y,
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
    mut query: Query<(&ServerEntity, &mut KinematicCharacterController, &Transform)>,
) {
    if let Some(endpoint) = server.get_endpoint_mut() {
        for connected_client_id in endpoint.clients() {
            while let Some(message) = endpoint.try_receive_message_from::<ClientMessage>(connected_client_id)
            {
                match message {
                    ClientMessage::Connected => {
                        let x = 50.0;
                        let y = 50.0;

                        commands
                            .spawn(RigidBody::Dynamic)
                            .insert(Collider::cuboid(8.0, 10.0))
                            .insert(KinematicCharacterController::default())
                            .insert(TransformBundle::from(Transform::from_xyz(x, y, 0.0)))
                            .insert(ServerEntity { client_id: connected_client_id });

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
