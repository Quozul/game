use std::string::ToString;

use bevy::prelude::*;

#[derive(Component)]
pub struct SinglePlayerButton;

#[derive(Component)]
pub struct JoinServerButton {
    pub input: Entity,
}

#[derive(Component)]
pub struct MenuItem;

#[derive(Component)]
pub struct JoinServerIp {
    pub(crate) focus: bool,
    pub(crate) ip: String,
}

const DEFAULT_IP: &str = "127.0.0.1";

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(Transform::default())
        .insert(MenuItem);

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(MenuItem)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "A game",
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    ..default()
                }),
            );

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Auto,
                        padding: UiRect::all(Val::Px(5.0)),
                        margin: UiRect::vertical(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .insert(SinglePlayerButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Single player",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    ));
                });

            let input = parent
                .spawn(
                    TextBundle::from_section(
                        DEFAULT_IP,
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                            font_size: 1000.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_text_alignment(TextAlignment::Center),
                )
                .insert(JoinServerIp {
                    ip: DEFAULT_IP.to_string(),
                    focus: false,
                })
                .id();

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Auto,
                        padding: UiRect::all(Val::Px(5.0)),
                        margin: UiRect::vertical(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .insert(JoinServerButton { input })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Join server",
                        TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    ));
                });
        });
}
