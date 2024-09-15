use bevy::prelude::*;

/// Used to help identify our player
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct VelocityDisplay(pub Entity);
