use crate::camera::components::Shake;
use crate::camera::events::TriggerCameraShakeEvent;
use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

pub fn camera_shake(
    mut q_camera: Query<(&mut Transform, &mut Shake), With<Camera>>,
    time: Res<Time>,
) {
    let delta = time.delta();
    let mut rng = rand::rng();

    for (mut camera_transform, mut shake) in q_camera.iter_mut() {
        if shake.intensity > 0. {
            let range = -shake.intensity..shake.intensity;
            let random_offset = Vec2::new(rng.random_range(range.clone()), rng.random_range(range));
            camera_transform.translation += random_offset.extend(0.);

            if let Some(remaining) = shake.remaining.checked_sub(delta) {
                shake.remaining = remaining;
            } else {
                shake.remaining = Duration::ZERO;
                shake.intensity = 0.;
            }
        }
    }
}

pub fn set_camera_shake(
    mut q_camera: Query<&mut Shake, With<Camera>>,
    mut events: EventReader<TriggerCameraShakeEvent>,
) {
    for event in events.read() {
        for mut shake in q_camera.iter_mut() {
            shake.remaining = event.duration;
            shake.intensity = event.intensity;
        }
    }
}
