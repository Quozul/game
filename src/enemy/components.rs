use bevy::prelude::{Component, Entity};

/// Used to help identify enemies
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct UiArrow(pub Entity);
