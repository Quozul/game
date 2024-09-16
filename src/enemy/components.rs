use bevy::prelude::Component;

/// Used to help identify enemies
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Health(pub u32);
