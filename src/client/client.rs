use std::net::{IpAddr, Ipv4Addr};
use std::ops::Deref;
use std::str::FromStr;
use std::thread;

use bevy::prelude::*;
use bevy_quinnet::client::certificate::CertificateVerificationMode;
use bevy_quinnet::client::connection::ConnectionConfiguration;
use bevy_quinnet::client::Client;
use bevy_quinnet::shared::ClientId;
use bevy_rapier2d::control::KinematicCharacterController;
use bevy_rapier2d::prelude::{Collider, RigidBody};

use shared::direction::Direction;
use shared::messages::{ClientMessage, ServerMessage};
use shared::server::server::start_server_app;

use crate::camera_follow::FollowSubject;
use crate::menu::{JoinServerButton, JoinServerIp, SinglePlayerButton};
use crate::{AppState, MyId};

fn join_server(next_state: &mut NextState<AppState>, client: &mut Client, server_ip: &str) {
    if let Ok(ip) = IpAddr::from_str(server_ip) {
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

        client
            .connection()
            .send_message(ClientMessage::Connected)
            .unwrap();

        next_state.set(AppState::InGame);
    }
}

pub(crate) fn start_client(
    mut next_state: ResMut<NextState<AppState>>,
    mut client: ResMut<Client>,
    query: Query<(&JoinServerButton, &Interaction), Changed<Interaction>>,
    ip_query: Query<&JoinServerIp>,
) {
    for (join_server_button, interaction) in &query {
        match *interaction {
            Interaction::Pressed => {
                let entity_result = ip_query.get(join_server_button.input);
                if let Ok(input) = entity_result {
                    join_server(&mut next_state, &mut client, input.ip.as_str());
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
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

pub(crate) fn start_server(
    mut next_state: ResMut<NextState<AppState>>,
    mut client: ResMut<Client>,
    query: Query<&Interaction, (Changed<Interaction>, With<SinglePlayerButton>)>,
) {
    for interaction in &query {
        match *interaction {
            Interaction::Pressed => {
                thread::spawn(|| {
                    start_server_app();
                });
                join_server(&mut next_state, &mut client, "127.0.0.1");
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn spawn_player(
    texture_atlases: &mut Assets<TextureAtlas>,
    commands: &mut Commands,
    asset_server: &AssetServer,
    id: ClientId,
    x: f32,
    y: f32,
) -> Entity {
    let texture_handle = asset_server.load("characters/player.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 37.0), 7, 16, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(37.0 / 2.0, 37.0 / 2.0))
        .insert(KinematicCharacterController::default())
        .insert(TransformBundle::from(Transform::from_xyz(x, y, 0.0)))
        .insert(ClientEntity { id })
        .insert(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..default()
        })
        .id()
}

pub fn controls(
    my_id: Res<MyId>,
    keys: Res<Input<KeyCode>>,
    mut client: ResMut<Client>,
    mut query: Query<(&ClientEntity, &mut KinematicCharacterController)>,
) {
    if let Some(connection) = client.get_connection_mut() {
        let direction = if keys.pressed(KeyCode::Z) {
            Some(Direction::Up)
        } else if keys.pressed(KeyCode::S) {
            Some(Direction::Down)
        } else if keys.pressed(KeyCode::D) {
            Some(Direction::Right)
        } else if keys.pressed(KeyCode::Q) {
            Some(Direction::Left)
        } else {
            None
        };

        if let Some(direction) = direction {
            let vec = direction.to_vec();

            connection
                .send_message(ClientMessage::Move { direction })
                .unwrap();

            for (client_entity, mut controller) in &mut query {
                if client_entity.id == my_id.id {
                    controller.translation = Some(vec);
                    break;
                }
            }
        }
    }
}

pub(crate) fn text_input(
    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<Input<KeyCode>>,
    mut query: Query<(&mut JoinServerIp, &mut Text)>,
) {
    for (mut join_server_ip, mut text_bundle) in &mut query {
        if kbd.just_pressed(KeyCode::Back) {
            join_server_ip.ip.pop();
        }

        for ev in evr_char.iter() {
            if !ev.char.is_control() {
                join_server_ip.ip.push(ev.char);
            }
        }

        let value = join_server_ip.ip.deref().to_string();
        text_bundle.sections.clear();
        text_bundle
            .sections
            .push(TextSection { value, ..default() });
    }
}

pub fn handle_server_messages(
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut client: ResMut<Client>,
    mut query: Query<(Entity, &ClientEntity, &mut Transform)>,
    mut my_id: ResMut<MyId>,
) {
    if let Some(connection) = client.get_connection_mut() {
        while let Ok(Some(message)) = connection.receive_message::<ServerMessage>() {
            match message {
                // Match on your own message types ...
                ServerMessage::Spawn { id, x, y } => {
                    let entity =
                        spawn_player(&mut texture_atlases, &mut commands, &asset_server, id, x, y);

                    if my_id.id == id {
                        my_id.entity = Some(entity);
                    }
                }
                ServerMessage::Position {
                    id,
                    translation,
                    rotation,
                } => {
                    for (_, client_entity, mut transform) in &mut query {
                        if client_entity.id == id {
                            transform.translation = translation;
                            transform.rotation = rotation;
                            break;
                        }
                    }
                }
                ServerMessage::YourId { id } => {
                    my_id.id = id;
                    println!("My id is {}", id);

                    for (entity, client_entity, _) in &mut query {
                        if client_entity.id == id {
                            my_id.entity = Some(entity);
                            println!("Found my entity");
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

#[derive(Component)]
pub struct ClientEntity {
    id: ClientId,
}
