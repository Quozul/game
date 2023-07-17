use crate::messages::ServerMessage;
use crate::server::message_events::{ClientConnectedEvent, ClientMoveEvent};
use crate::server_entities::NetworkServerEntity;
use bevy::prelude::*;
use bevy_quinnet::server::Server;
use bevy_rapier2d::prelude::{Collider, KinematicCharacterController, RigidBody};

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
                .spawn(RigidBody::KinematicPositionBased)
                .insert(Collider::cuboid(37.0 / 2.0, 37.0 / 2.0))
                .insert(KinematicCharacterController::default())
                .insert(TransformBundle::from(Transform::from_xyz(x, y, 0.0)))
                .insert(NetworkServerEntity {
                    client_id: event.client_id,
                });

            endpoint
                .send_message(
                    event.client_id,
                    ServerMessage::YourId {
                        id: event.client_id,
                    },
                )
                .unwrap();

            // Send all players to the new player
            for (server_entity, transform) in &mut query {
                endpoint
                    .send_message(
                        event.client_id,
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
                            id: event.client_id,
                            x,
                            y,
                        },
                    )
                    .unwrap();
            }
        }
    }
}

pub(crate) fn handle_client_move(
    mut client_connected_reader: EventReader<ClientMoveEvent>,
    mut query: Query<(&NetworkServerEntity, &mut KinematicCharacterController)>,
) {
    for event in client_connected_reader.iter() {
        for (server_entity, mut controller) in &mut query {
            if server_entity.client_id == event.client_id {
                let vel = event.direction.to_vec();
                controller.translation = Some(vel);
                break;
            }
        }
    }
}
