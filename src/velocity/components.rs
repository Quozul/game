use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
pub struct Force(pub Vec2);

#[derive(Component)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Bundle, Default)]
pub struct RigidBodyBundle {
    velocity: Velocity,
    force: Force,
    mass: Mass,
}
