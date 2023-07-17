#![feature(async_closure)]
#![windows_subsystem = "windows"]

use crate::camera_follow::camera_follow;
use bevy::prelude::*;
use bevy_quinnet::client::QuinnetClientPlugin;
use bevy_quinnet::shared::ClientId;
use bevy_rapier2d::prelude::{
    NoUserData, RapierConfiguration, RapierDebugRenderPlugin, RapierPhysicsPlugin, TimestepMode,
    Vect,
};
use shared::gravity::gravity;
use shared::server::server::spawn_floor;
use shared::server_entities::StaticServerEntity;
use shared::FIXED_TIMESTEP;

use crate::client::{
    controls, handle_server_messages, setup_in_game, start_client, start_server, text_input,
};
use crate::menu::{setup_menu, MenuItem};

mod camera_follow;
mod client;
mod menu;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}

#[derive(Resource)]
pub struct MyId {
    id: ClientId,
    entity: Option<Entity>,
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
            gravity: Vect::new(0.0, -1.0),
            timestep_mode: TimestepMode::Fixed {
                dt: FIXED_TIMESTEP,
                substeps: 1,
            },
            ..default()
        })
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .insert_resource(MyId {
            id: 0,
            entity: None,
        })
        .insert_resource(StaticServerEntity::default())
        .add_state::<AppState>()
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(OnExit(AppState::Menu), cleanup_menu)
        .add_systems(OnEnter(AppState::InGame), (setup_in_game, spawn_floor))
        .add_systems(
            Update,
            (start_client, start_server, camera_follow, text_input),
        )
        .add_systems(FixedUpdate, (controls, handle_server_messages, gravity))
        .run();
}

fn cleanup_menu(mut commands: Commands, query: Query<(Entity, &MenuItem)>) {
    for (entity, _) in &query {
        commands.entity(entity).despawn_recursive()
    }
}
