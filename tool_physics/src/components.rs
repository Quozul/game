use bevy::math::Vec2;
use bevy::prelude::*;

/// Gives the Entity the ability to move
#[derive(Component, Default)]
pub struct Velocity {
    pub linear_velocity: Vec2,
    pub angular_velocity: f32,
}

impl Velocity {
    pub fn linear(linear_velocity: Vec2) -> Self {
        Self {
            linear_velocity,
            ..default()
        }
    }
}

/// Move the Entity with an external force
#[derive(Component, Default)]
pub struct Force {
    pub linear_force: Vec2,
}

/// Move the Entity with an external impulse
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
    rigid_body_type: RigidBodyType,
}

impl RigidBodyBundle {
    pub fn with_drag_coefficient(mut self, drag_coefficient: DragCoefficient) -> Self {
        self.drag_coefficient = drag_coefficient;
        self
    }

    pub fn with_initial_velocity(mut self, initial_velocity: Velocity) -> Self {
        self.velocity = initial_velocity;
        self
    }

    pub fn with_mass(mut self, mass: f32) -> Self {
        self.mass = Mass(mass);
        self
    }
}

#[derive(Component, PartialEq)]
pub enum RigidBodyType {
    /// A Dynamic body can move and respond to collisions
    Dynamic,
    /// A Static body doesn't move and doesn't respond to collisions but other bodies colliding with this one can
    Static,
}

impl Default for RigidBodyType {
    fn default() -> Self {
        Self::Dynamic
    }
}

#[derive(Component)]
pub struct Sensor;
