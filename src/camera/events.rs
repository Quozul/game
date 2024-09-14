use bevy::prelude::Event;
use std::time::Duration;

#[derive(Event)]
pub struct TriggerCameraShakeEvent {
    pub duration: Duration,
    pub intensity: f32,
}
