use bevy::prelude::*;
use std::time::Duration;

/// Used to help identify our player
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct VelocityDisplay(pub Entity);

#[derive(Component, Default)]
pub struct Cooldown(pub Duration);
