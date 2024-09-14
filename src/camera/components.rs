use bevy::prelude::{Component, Entity};
use std::time::Duration;

#[derive(Component)]
pub struct CameraFollow(pub Entity);

#[derive(Component, Default)]
pub struct Shake {
    pub remaining: Duration,
    pub intensity: f32,
}
