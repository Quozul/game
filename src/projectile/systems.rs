use crate::camera::events::TriggerCameraShakeEvent;
use crate::camera::main_camera::MainCamera;
use crate::projectile::components::{Cannon, Life, Projectile};
use crate::utils::calculate_rotation_angle::calculate_direction_angle;
use crate::utils::get_mouse_world_position::get_mouse_world_position_from_queries;
use crate::velocity::components::{RigidBodyBundle, Velocity};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;
use std::time::Duration;

pub fn shoot_bullets(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut q_cannons: Query<(&mut Cannon, &Transform)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut trigger_camera_shake_events: EventWriter<TriggerCameraShakeEvent>,
) {
    if let Some(world_position) = get_mouse_world_position_from_queries(q_windows, q_cameras)
        && mouse_input.pressed(MouseButton::Left)
    {
        // Only iter through cannons that are ready to fire
        // TODO: Trigger event for spawning projectile
        for (mut cannon, transform) in q_cannons
            .iter_mut()
            .filter(|cannon| cannon.0.cooldown == Duration::ZERO)
        {
            // Spawn a new projectile
            let bullet_mesh = Mesh2dHandle(cannon.mesh_handle.clone());
            let player_translation = transform.translation.xy();
            let angle = calculate_direction_angle(player_translation, world_position);

            // let angle = Vec2::from_angle(25.0);
            let initial_velocity = angle * 500.;
            // debug!("angle {} initial_velocity {}", angle, initial_velocity);

            // TODO: Create a projectile bundle
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: bullet_mesh,
                    material: cannon.material_handle.clone(),
                    transform: Transform::from_translation(
                        transform.translation + cannon.offset + angle.extend(0.) * 50.,
                    ),
                    ..Default::default()
                },
                RigidBodyBundle::new(1.0, initial_velocity),
                Projectile,
                Life::default(),
            ));

            // Shaking the camera acts as a way to spread the projectiles
            /* trigger_camera_shake_events.send(TriggerCameraShakeEvent {
                duration: Duration::from_millis(200),
                intensity: 2.0,
            });*/

            // Reset the cooldown once the cannon has fired
            cannon.cooldown = Duration::from_millis(200);
        }
    }
}

pub fn cannon_cooldown(mut q_cannons: Query<&mut Cannon>, time: Res<Time>) {
    let delta = time.delta();

    for mut cannon in q_cannons.iter_mut() {
        if let Some(remaining) = cannon.cooldown.checked_sub(delta) {
            cannon.cooldown = remaining;
        } else {
            cannon.cooldown = Duration::ZERO;
        }
    }
}

pub fn remove_bullets(
    mut commands: Commands,
    q_projectiles: Query<(&Velocity, &Life, Entity), With<Projectile>>,
) {
    for (velocity, life, entity) in q_projectiles.iter() {
        if velocity.0.length() < 1. {
            let seconds = life.0.as_millis();
            debug!("Removed bullet lived for {seconds}ms");
            commands.entity(entity).despawn();
        }
    }
}

pub fn increment_life(mut q_lives: Query<&mut Life>, time: Res<Time>) {
    for mut life in q_lives.iter_mut() {
        life.0 += time.delta();
    }
}
