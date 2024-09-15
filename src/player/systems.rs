use crate::camera::main_camera::MainCamera;
use crate::physics::components::{DragCoefficient, Force, Mass, Velocity};
use crate::physics::utils::get_terminal_velocity;
use crate::player::components::{Player, VelocityDisplay};
use crate::utils::calculate_rotation_angle::calculate_rotation_angle;
use crate::utils::get_mouse_world_position::get_mouse_world_position_from_queries;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const THROTTLE: f32 = 500.0;

pub fn move_player(
    mut rectangles: Query<&mut Force, With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut force) = rectangles.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    let throttle = if kb_input.pressed(KeyCode::ShiftLeft) {
        THROTTLE * 5.0
    } else {
        THROTTLE
    };

    force.linear_force = direction.normalize_or_zero() * throttle;
}

pub fn rotate_towards_mouse(
    mut rectangles: Query<&mut Transform, With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Ok(mut player_transform) = rectangles.get_single_mut() else {
        return;
    };

    if let Some(world_position) = get_mouse_world_position_from_queries(q_windows, q_camera) {
        let player_translation = player_transform.translation.xy();
        let target_angle = calculate_rotation_angle(player_translation, world_position);

        let current_angle = player_transform.rotation;
        player_transform.rotation = current_angle.lerp(target_angle, 0.1);
    }
}

pub fn update_velocity_display(
    q_velocity_displays: Query<(&Velocity, &Force, &DragCoefficient, &Mass, &VelocityDisplay)>,
    mut q_texts: Query<&mut Text>,
) {
    for (velocity, force, drag, mass, display) in q_velocity_displays.iter() {
        if let Ok(mut text) = q_texts.get_mut(display.0) {
            let current_speed = velocity.linear_velocity.length();
            let maximum_speed = get_terminal_velocity(mass.0, force.linear_force.length(), drag.0);
            let percentage = current_speed / maximum_speed * 100.0;
            text.sections[1].value = format!(
                "{:.0}/{:.0} {:.0}%",
                current_speed, maximum_speed, percentage
            );
        }
    }
}
