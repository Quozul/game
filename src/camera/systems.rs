use crate::camera::components::{CameraFollow, MainCamera};
use crate::utils::get_mouse_world_position::get_mouse_world_position;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

pub fn camera_follow(
    mut q_camera: Query<(&mut Transform, &CameraFollow), With<Camera>>,
    q_players: Query<&Transform, Without<Camera>>,
) {
    for (mut camera_transform, follow) in q_camera.iter_mut() {
        if let Ok(player_transform) = q_players.get(follow.0) {
            let target_position = player_transform.translation;

            let lerp_factor = 0.1; // Lower lerp results in more lag behind the target position
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

const SHAKE_AMOUNT: f32 = 10.0;

pub fn camera_shake(mut q_camera: Query<&mut Transform, With<Camera>>) {
    let mut camera_transform = q_camera.single_mut();
    let mut rng = rand::thread_rng();
    let random_offset = Vec2::new(
        rng.gen_range(-SHAKE_AMOUNT..SHAKE_AMOUNT),
        rng.gen_range(-SHAKE_AMOUNT..SHAKE_AMOUNT),
    );
    camera_transform.translation += random_offset.extend(0.);
}
