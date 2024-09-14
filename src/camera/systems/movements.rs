use crate::camera::components::CameraFollow;
use crate::camera::main_camera::MainCamera;
use crate::utils::get_mouse_world_position::get_mouse_world_position;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn camera_follow(
    mut q_camera: Query<(&mut Transform, &CameraFollow), With<Camera>>,
    q_players: Query<&Transform, Without<Camera>>,
) {
    for (mut camera_transform, follow) in q_camera.iter_mut() {
        if let Ok(player_transform) = q_players.get(follow.0) {
            let target_position = player_transform.translation;

            let lerp_factor = 0.05; // Lower lerp results in more lag behind the target position
            camera_transform.translation = camera_transform
                .translation
                .lerp(target_position, lerp_factor);
        }
    }
}

pub fn camera_offset(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut q_camera: Query<(&Camera, &mut Transform, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, mut camera_transform, global_transform) = q_camera.single_mut();
    let window = q_windows.single();

    if let Some(mouse_position) = get_mouse_world_position(camera, global_transform, window) {
        let offset = mouse_position - camera_transform.translation.xy();
        let offset = offset.normalize_or_zero() * 1.0;
        camera_transform.translation += offset.extend(0.);
    }
}
