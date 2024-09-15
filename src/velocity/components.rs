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

#[derive(Component)]
pub struct DragCoefficient(pub f32);

impl DragCoefficient {
    pub const CIRCLE: Self = Self(0.47);
    pub const CUBE: Self = Self(1.05);
}

impl Default for DragCoefficient {
    fn default() -> Self {
        Self::CIRCLE
    }
}

#[derive(Bundle, Default)]
pub struct RigidBodyBundle {
    velocity: Velocity,
    force: Force,
    mass: Mass,
    drag_coefficient: DragCoefficient,
}

impl RigidBodyBundle {
    pub fn new(mass: f32, initial_velocity: Vec2, drag_coefficient: f32) -> Self {
        Self {
            mass: Mass(mass),
            velocity: Velocity(initial_velocity), // Kind of acts as a initial impulse
            drag_coefficient: DragCoefficient(drag_coefficient),
            ..default()
        }
    }
}
