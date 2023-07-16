use std::net::{IpAddr, Ipv4Addr};
use std::thread;

use bevy::prelude::*;
use bevy_quinnet::client::certificate::CertificateVerificationMode;
use bevy_quinnet::client::connection::ConnectionConfiguration;
use bevy_quinnet::client::Client;
use bevy_quinnet::shared::ClientId;

use shared::direction::Direction;
use shared::messages::{ClientMessage, ServerMessage};
use shared::server::start_server_app;

use crate::menu::{JoinServerButton, SinglePlayerButton};
use crate::AppState;

fn join_server(next_state: &mut NextState<AppState>, client: &mut Client) {
    client
        .open_connection(
            ConnectionConfiguration::from_ips(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
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

pub(crate) fn start_client(
    mut next_state: ResMut<NextState<AppState>>,
    mut client: ResMut<Client>,
    query: Query<&Interaction, (Changed<Interaction>, With<JoinServerButton>)>,
) {
    for interaction in &query {
        match *interaction {
            Interaction::Pressed => {
                join_server(&mut next_state, &mut client);
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
        });
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
                join_server(&mut next_state, &mut client);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[derive(Component)]
pub struct FollowSubject(Entity);

pub fn camera_follow(
    mut camera_query: Query<(&FollowSubject, &mut Transform), With<Camera>>,
    player_query: Query<&Transform, Without<Camera>>,
) {
    for (subject, mut transform) in &mut camera_query {
        let player = player_query.get(subject.0);

        if let Ok(player_transform) = player {
            transform.translation = player_transform.translation;
            transform.translation.z = 999.0;
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
) {
    let texture_handle = asset_server.load("characters/player.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(48.0, 48.0),
        6,
        10,
        None,
        Some(Vec2::new(0.0, 8.0)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..default()
        })
        .insert(Transform {
            translation: Vec3::new(x, y, 5.0), // Change Z axis to make it visible
            ..default()
        })
        .insert(ClientEntity { id });
}

pub fn controls(keys: Res<Input<KeyCode>>, mut client: ResMut<Client>) {
    if let Some(connection) = client.get_connection_mut() {
        if keys.pressed(KeyCode::Z) {
            connection
                .send_message(ClientMessage::Move {
                    direction: Direction::Up,
                })
                .unwrap();
        } else if keys.pressed(KeyCode::S) {
            connection
                .send_message(ClientMessage::Move {
                    direction: Direction::Down,
                })
                .unwrap();
        } else if keys.pressed(KeyCode::D) {
            connection
                .send_message(ClientMessage::Move {
                    direction: Direction::Right,
                })
                .unwrap();
        } else if keys.pressed(KeyCode::Q) {
            connection
                .send_message(ClientMessage::Move {
                    direction: Direction::Left,
                })
                .unwrap();
        }
    }
}

pub fn handle_server_messages(
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut client: ResMut<Client>,
    mut query: Query<(&ClientEntity, &mut Transform)>,
) {
    if let Some(connection) = client.get_connection_mut() {
        while let Ok(Some(message)) = connection.receive_message::<ServerMessage>() {
            match message {
                // Match on your own message types ...
                ServerMessage::Spawn { id, x, y } => {
                    spawn_player(&mut texture_atlases, &mut commands, &asset_server, id, x, y);
                }
                ServerMessage::Position { id, x, y } => {
                    for (client_entity, mut transform) in &mut query {
                        if client_entity.id == id {
                            transform.translation.x = x;
                            transform.translation.y = y;
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
