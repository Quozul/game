use bevy::prelude::*;
use bevy_quinnet::server::Server;
use bevy_rapier2d::prelude::{Collider, KinematicCharacterController, RigidBody};

use crate::direction::{Direction, FacingDirection, Move};
use crate::gravity::apply_force;
use crate::messages::ServerMessage;
use crate::server::message_events::{ClientConnectedEvent, ClientMoveEvent};
use crate::server_entities::NetworkServerEntity;

pub(crate) fn handle_client_connected(
    mut server: ResMut<Server>,
    mut commands: Commands,
    mut client_connected_reader: EventReader<ClientConnectedEvent>,
    mut query: Query<(&NetworkServerEntity, &Transform)>,
) {
    if let Some(endpoint) = server.get_endpoint_mut() {
        for event in client_connected_reader.iter() {
            let x = 50.0;
            let y = 50.0;

            // Spawn the player
            commands
                .spawn(RigidBody::KinematicVelocityBased)
                .insert(Collider::cuboid(8.0, 8.0))
                .insert(KinematicCharacterController {
                    autostep: None,
                    ..default()
                })
                .insert(TransformBundle::from(Transform::from_xyz(x, y, 0.0)))
                .insert(NetworkServerEntity {
                    client_id: event.client_id,
                })
                .insert(Move {
                    direction: Direction::Idling {
                        direction: FacingDirection::Down,
                    },
                });

            // Send all players to the new player
            for (server_entity, transform) in &mut query {
                endpoint.try_send_message(
                    event.client_id,
                    ServerMessage::Spawn {
                        id: server_entity.client_id,
                        x: transform.translation.x,
                        y: transform.translation.y,
                        you: event.client_id == server_entity.client_id,
                    },
                );
            }

            // Send the new player to all players
            for client_id in endpoint.clients() {
                endpoint
                    .send_message(
                        client_id,
                        ServerMessage::Spawn {
                            id: event.client_id,
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

pub(crate) fn handle_move(mut query: Query<(&mut KinematicCharacterController, &Move)>) {
    for (mut controller, move_component) in &mut query {
        let vel = move_component.direction.to_vec();
        apply_force(&mut controller, vel);
    }
}

pub(crate) fn handle_client_move(
    mut server: ResMut<Server>,
    mut client_connected_reader: EventReader<ClientMoveEvent>,
    mut query: Query<(&NetworkServerEntity, &mut Move)>,
) {
    for event in client_connected_reader.iter() {
        for (server_entity, mut move_component) in &mut query {
            if server_entity.client_id == event.client_id {
                move_component.direction = event.direction;
                break;
            }
        }

        if let Some(endpoint) = server.get_endpoint_mut() {
            // Send the direction to all players
            for client_id in endpoint.clients() {
                endpoint.try_send_message(
                    client_id,
                    ServerMessage::Direction {
                        id: event.client_id,
                        direction: event.direction,
                    },
                );
            }
        }
    }
}
