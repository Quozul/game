use std::net::{IpAddr, Ipv4Addr};
use std::ops::Deref;
use std::str::FromStr;
use std::thread;

use bevy::prelude::*;
use bevy_quinnet::client::certificate::CertificateVerificationMode;
use bevy_quinnet::client::connection::ConnectionConfiguration;
use bevy_quinnet::client::Client;
use bevy_quinnet::shared::ClientId;

use shared::messages::{ClientMessage, ServerMessage};
use shared::server::server::start_server_app;

use crate::camera_follow::FollowSubject;
use crate::menu::{JoinServerButton, JoinServerIp, SinglePlayerButton};
use crate::message_handlers::spawn_player::SpawnPlayerEvent;
use crate::message_handlers::update_direction::UpdateDirection;
use crate::message_handlers::update_position::UpdatePositionEvent;
use crate::message_handlers::update_your_id::UpdateYourId;
use crate::AppState;

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

pub(crate) fn handle_server_messages(
    mut client: ResMut<Client>,
    mut spawn_player_event_writer: EventWriter<SpawnPlayerEvent>,
    mut update_position_event_writer: EventWriter<UpdatePositionEvent>,
    mut update_your_id_event_writer: EventWriter<UpdateYourId>,
    mut update_direction_event_writer: EventWriter<UpdateDirection>,
) {
    if let Some(connection) = client.get_connection_mut() {
        while let Ok(Some(message)) = connection.receive_message::<ServerMessage>() {
            match message {
                // Match on your own message types ...
                ServerMessage::Spawn { id, x, y } => {
                    spawn_player_event_writer.send(SpawnPlayerEvent { id, x, y })
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
                ServerMessage::YourId { id } => {
                    update_your_id_event_writer.send(UpdateYourId { id });
                }
                ServerMessage::Direction { id, direction } => {
                    update_direction_event_writer.send(UpdateDirection { id, direction });
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
