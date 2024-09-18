use crate::camera::main_camera::MainCamera;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn get_mouse_world_position_from_queries(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    let (camera, camera_transform) = q_camera.single();
    q_windows
        .get_single()
        .ok()
        .and_then(|window| get_mouse_world_position(camera, camera_transform, window))
}

pub fn get_mouse_world_position(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    window: &Window,
) -> Option<Vec2> {
    window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
}
