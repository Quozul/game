use crate::camera::events::TriggerCameraShakeEvent;
use crate::camera::systems::movements::{camera_follow, camera_offset, camera_zoom};
use crate::camera::systems::shake::{camera_shake, set_camera_shake};
use crate::AppState;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TriggerCameraShakeEvent>().add_systems(
            Update,
            (
                camera_follow,
                camera_zoom,
                camera_offset,
                camera_shake,
                set_camera_shake,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
