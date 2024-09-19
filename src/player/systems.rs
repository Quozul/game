use crate::camera::events::TriggerCameraShakeEvent;
use crate::camera::main_camera::MainCamera;
use crate::inventory::components::Inventory;
use crate::physics::components::{DragCoefficient, Force, Impulse, Mass, Velocity};
use crate::physics::resources::PhysicsResource;
use crate::physics::utils::get_terminal_velocity::get_terminal_velocity;
use crate::player::components::{Cooldown, Player, VelocityDisplay};
use crate::projectile::cannon_bundle::Cannon;
use crate::projectile::projectile_bundle::ProjectileBundle;
use crate::utils::calculate_rotation_angle::{calculate_direction_angle, calculate_rotation_angle};
use crate::utils::get_mouse_world_position::get_mouse_world_position_from_queries;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::time::Duration;

const THROTTLE: f32 = 500.0;
const BOOST_MULTIPLIER: f32 = 50.0;

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
    mut q_cannons: Query<(&Inventory<Cannon>, &Transform, &mut Impulse, &mut Cooldown)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut trigger_camera_shake_events: EventWriter<TriggerCameraShakeEvent>,
) {
    if let Some(world_position) = get_mouse_world_position_from_queries(q_windows, q_cameras)
        && mouse_input.pressed(MouseButton::Left)
    {
        // Only iter through cannons that are ready to fire
        // TODO: Trigger event for spawning projectile
        for (inventory, origin, mut impulse, mut cooldown) in q_cannons.iter_mut() {
            let player_translation = origin.translation.xy();
            let angle = calculate_direction_angle(player_translation, world_position);
            if let Some(property) = inventory.get_selected_item() {
                if cooldown.0 != Duration::ZERO {
                    continue;
                }

                commands.spawn(ProjectileBundle::from_cannon(origin, property, angle));

                // Shaking the camera acts as a way to spread the projectiles
                trigger_camera_shake_events.send(TriggerCameraShakeEvent {
                    duration: Duration::from_millis(property.reload),
                    intensity: property.spread,
                });

                // Simulate recoil, we need to add in case the previous impulse has not been processed yet
                impulse.linear_impulse += -angle * property.recoil;

                // Reset the cooldown once the cannon has fired
                cooldown.0 = Duration::from_millis(property.reload);
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
    physics_resource: Res<PhysicsResource>,
    q_velocity_displays: Query<(&Velocity, &Force, &Mass, &DragCoefficient, &VelocityDisplay)>,
    mut q_texts: Query<&mut Text>,
) {
    for (velocity, force, mass, drag, display) in q_velocity_displays.iter() {
        if let Ok(mut text) = q_texts.get_mut(display.0) {
            let current_speed = velocity.linear_velocity.length();
            let maximum_speed = get_terminal_velocity(
                physics_resource.air_density,
                mass.0,
                force.linear_force.length(),
                drag.0,
            );
            let percentage = current_speed / maximum_speed * 100.0;
            text.sections[1].value = format!(
                "{:.0}/{:.0} {:.0}%",
                current_speed, maximum_speed, percentage
            );
        }
    }
}
