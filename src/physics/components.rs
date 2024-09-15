use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity {
    pub linear_velocity: Vec2,
}

#[derive(Component, Default)]
pub struct Force {
    pub linear_force: Vec2,
}

#[derive(Component, Default)]
pub struct Impulse {
    pub linear_impulse: Vec2,
}

#[derive(Component)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.0)
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
    mass: Mass,
    drag_coefficient: DragCoefficient,
}

impl RigidBodyBundle {
    pub fn new(mass: f32, initial_velocity: Vec2, drag_coefficient: f32) -> Self {
        Self {
            mass: Mass(mass),
            velocity: Velocity {
                linear_velocity: initial_velocity,
            }, // Kind of acts as an initial impulse
            drag_coefficient: DragCoefficient(drag_coefficient),
        }
    }

    pub fn with_drag_coefficient(mut self, drag_coefficient: DragCoefficient) -> Self {
        self.drag_coefficient = drag_coefficient;
        self
    }
}
