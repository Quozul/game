#![feature(async_closure)]

use bevy::prelude::*;
use bevy_quinnet::client::QuinnetClientPlugin;
use bevy_rapier2d::prelude::{
    NoUserData, RapierConfiguration, RapierDebugRenderPlugin, RapierPhysicsPlugin, Vect,
};

use crate::client::{
    camera_follow, controls, handle_server_messages, setup_in_game, start_client, start_server,
};
use crate::menu::{setup_menu, MenuItem};

mod client;
mod menu;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            QuinnetClientPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vect::new(0.0, 0.0),
            ..default()
        })
        .add_state::<AppState>()
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(OnExit(AppState::Menu), cleanup_menu)
        .add_systems(OnEnter(AppState::InGame), setup_in_game)
        .add_systems(
            Update,
            (
                start_client,
                start_server,
                handle_server_messages,
                camera_follow,
                controls,
            ),
        )
        .run();
}

fn cleanup_menu(mut commands: Commands, query: Query<(Entity, &MenuItem)>) {
    for (entity, _) in &query {
        commands.entity(entity).despawn_recursive()
    }
}
