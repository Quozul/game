use crate::camera::systems::{camera_follow, camera_offset, camera_shake};
use crate::AppState;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (camera_follow, camera_offset /*, camera_shake*/).run_if(in_state(AppState::InGame)),
        );
    }
}
