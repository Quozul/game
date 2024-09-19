use bevy::prelude::Component;
use std::time::Duration;

#[derive(Component)]
pub struct Damage(pub u32);

#[derive(Component, Default)]
pub struct Lifetime(pub Duration);
