use crate::camera::events::TriggerCameraShakeEvent;
use crate::camera::main_camera::MainCamera;
use crate::inventory::components::Inventory;
use crate::player::components::{Cooldown, Player, VelocityDisplay};
use crate::utils::calculate_rotation_angle::{calculate_direction_angle, calculate_rotation_angle};
use crate::utils::get_mouse_world_position::get_mouse_world_position_from_queries;
use crate::weapon::weapon_data::Weapon;
use crate::AIR_DENSITY;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::time::Duration;
use tool_physics::get_terminal_velocity::get_terminal_velocity;
use tool_physics::{DragCoefficient, Force, Impulse, Mass, Velocity};

const THROTTLE: f32 = 500.0;
const BOOST_MULTIPLIER: f32 = 2.0;

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
        THROTTLE * BOOST_MULTIPLIER
    } else {
        THROTTLE
    };

    force.linear_force = direction.normalize_or_zero() * throttle;
}

pub fn shoot_bullets(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut q_cannons: Query<(&Inventory<Weapon>, &Transform, &mut Impulse, &mut Cooldown)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut trigger_camera_shake_events: EventWriter<TriggerCameraShakeEvent>,
) {
    if let Some(world_position) = get_mouse_world_position_from_queries(q_windows, q_cameras)
        && mouse_input.pressed(MouseButton::Left)
    {
        // Only iter through cannons that are ready to fire
        // TODO: Trigger event for spawning projectile
        for (inventory, player_transform, mut impulse, mut cooldown) in q_cannons.iter_mut() {
            let angle =
                calculate_direction_angle(player_transform.translation.xy(), world_position);
            if let Some(cannon) = inventory.get_selected_item() {
                if cooldown.0 != Duration::ZERO {
                    continue;
                }

                let offset = player_transform.rotation.mul_vec3(cannon.offset);
                let origin = Transform::from_translation(player_transform.translation + offset);
                commands.spawn(cannon.projectile.create_projectile(origin, angle));

                // Shaking the camera acts as a way to spread the projectiles
                trigger_camera_shake_events.send(TriggerCameraShakeEvent {
                    duration: cannon.reload,
                    intensity: cannon.spread,
                });

                // Simulate recoil, we need to add in case the previous impulse has not been processed yet
                impulse.linear_impulse += -angle * cannon.recoil;

                // Reset the cooldown once the cannon has fired
                cooldown.0 = cannon.reload;
            }
        }
    }
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
    q_velocity_displays: Query<(&Velocity, &Force, &Mass, &DragCoefficient, &VelocityDisplay)>,
    mut q_texts: Query<&mut Text>,
) {
    for (velocity, force, mass, drag, display) in q_velocity_displays.iter() {
        if let Ok(mut text) = q_texts.get_mut(display.0) {
            let current_speed = velocity.linear_velocity.length();
            let maximum_speed =
                get_terminal_velocity(AIR_DENSITY, mass.0, force.linear_force.length(), drag.0);
            let percentage = current_speed / maximum_speed * 100.0;
            text.sections[1].value = format!(
                "{:.0}/{:.0} {:.0}%",
                current_speed, maximum_speed, percentage
            );
        }
    }
}
