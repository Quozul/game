use crate::camera::events::TriggerCameraShakeEvent;
use crate::camera::main_camera::MainCamera;
use crate::enemy::components::{Enemy, Health};
use crate::physics::events::CollisionEvent;
use crate::physics::movements_components::Impulse;
use crate::projectile::components::{Cannon, Damage, Lifetime};
use crate::projectile::projectile_bundle::ProjectileBundle;
use crate::utils::calculate_rotation_angle::calculate_direction_angle;
use crate::utils::get_mouse_world_position::get_mouse_world_position_from_queries;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::time::Duration;

pub fn shoot_bullets(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut q_cannons: Query<(&mut Cannon, &Transform, &mut Impulse)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut trigger_camera_shake_events: EventWriter<TriggerCameraShakeEvent>,
) {
    if let Some(world_position) = get_mouse_world_position_from_queries(q_windows, q_cameras)
        && mouse_input.pressed(MouseButton::Left)
    {
        // Only iter through cannons that are ready to fire
        // TODO: Trigger event for spawning projectile
        for (mut cannon, origin, mut impulse) in q_cannons.iter_mut() {
            let player_translation = origin.translation.xy();
            let angle = calculate_direction_angle(player_translation, world_position);

            for property in &mut cannon.properties {
                if property.cooldown != Duration::ZERO {
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
                property.cooldown = Duration::from_millis(property.reload);
            }
        }
    }
}

pub fn cannon_cooldown(mut q_cannons: Query<&mut Cannon>, time: Res<Time>) {
    let delta = time.delta();

    for mut cannon in q_cannons.iter_mut() {
        for property in &mut cannon.properties {
            if let Some(remaining) = property.cooldown.checked_sub(delta) {
                property.cooldown = remaining;
            } else {
                property.cooldown = Duration::ZERO;
            }
        }
    }
}

const MAX_PROJECTILE_LIFE: u64 = 1;

pub fn remove_bullets_of_old_age(
    mut commands: Commands,
    q_projectiles: Query<(&Lifetime, Entity), With<Damage>>,
) {
    for (velocity, entity) in q_projectiles.iter() {
        if velocity.0.as_secs() > MAX_PROJECTILE_LIFE {
            commands.entity(entity).despawn();
        }
    }
}

pub fn remove_bullets_on_collision(
    mut command: Commands,
    mut event: EventReader<CollisionEvent>,
    q_projectiles: Query<&Damage>,
    mut q_enemies: Query<&mut Health, With<Enemy>>,
) {
    for ev in event.read() {
        handle_collision(
            &mut command,
            &q_projectiles,
            &mut q_enemies,
            ev.first,
            ev.second,
        );
        handle_collision(
            &mut command,
            &q_projectiles,
            &mut q_enemies,
            ev.second,
            ev.first,
        );
    }
}

fn handle_collision(
    command: &mut Commands,
    q_projectiles: &Query<&Damage>,
    q_enemies: &mut Query<&mut Health, With<Enemy>>,
    projectile_entity: Entity,
    enemy_entity: Entity,
) {
    if let Ok(projectile) = q_projectiles.get(projectile_entity) {
        command.entity(projectile_entity).despawn();

        if let Ok(mut health) = q_enemies.get_mut(enemy_entity) {
            health.0 = health.0.saturating_sub(projectile.0);
        }
    }
}

pub fn increment_lifetime(mut q_lifetimes: Query<&mut Lifetime>, time: Res<Time>) {
    for mut life_time in q_lifetimes.iter_mut() {
        life_time.0 += time.delta();
    }
}
