use crate::map::setup::{setup_map, setup_tutorial};
use crate::AppState;
use bevy::app::App;
use bevy::prelude::{OnEnter, Plugin};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), (setup_map, setup_tutorial));
    }
}
