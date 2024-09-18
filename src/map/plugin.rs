use crate::map::setup::{setup_map, setup_tutorial};
use crate::map::systems::draw_world;
use crate::AppState;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), (setup_map, setup_tutorial))
            .add_systems(Update, draw_world);
    }
}
