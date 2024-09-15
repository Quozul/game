use crate::physics::components::{DragCoefficient, Force, Impulse, Mass, Velocity};
use bevy::prelude::*;

const FLUID_DENSITY: f32 = 0.05; // 1.2 kg/m^3 at room temperature
const CROSS_SECTIONAL_AREA: f32 = 1.0;

pub fn apply_acceleration(mut q_velocity: Query<(&mut Velocity, &Force, &Mass)>, time: Res<Time>) {
    let delta_time = time.delta_seconds();

    for (mut velocity, applied_force, mass) in q_velocity.iter_mut() {
        let acceleration = applied_force.linear_force / mass.0;
        velocity.linear_velocity += acceleration * delta_time;
    }
}

pub fn apply_drag(
    mut q_velocity: Query<(&mut Velocity, &Mass, &DragCoefficient)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (mut velocity, mass, drag_coefficient) in q_velocity.iter_mut() {
        let air_resistance = get_drag_force(velocity.linear_velocity, drag_coefficient.0) / mass.0;
        velocity.linear_velocity -= air_resistance * delta_time;
    }
}

pub fn move_object(mut q_velocity: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    let delta_time = time.delta_seconds();

    for (mut transform, velocity) in q_velocity.iter_mut() {
        transform.translation += velocity.linear_velocity.extend(0.) * delta_time;
    }
}

/// Applies an impulse to the velocity then resets the impulse
pub fn update_impulse(mut q_velocity: Query<(&mut Velocity, &mut Impulse)>, time: Res<Time>) {
    let delta_time = time.delta_seconds();

    for (mut velocity, mut impulse) in q_velocity.iter_mut() {
        velocity.linear_velocity += impulse.linear_impulse * delta_time;
        impulse.linear_impulse = Vec2::ZERO;
    }
}

fn get_drag_force(linear_velocity: Vec2, drag_coefficient: f32) -> Vec2 {
    0.5 * FLUID_DENSITY
        * linear_velocity.normalize_or_zero()
        * linear_velocity.length_squared()
        * drag_coefficient
        * CROSS_SECTIONAL_AREA
}

#[allow(dead_code)]
fn get_terminal_velocity(mass: f32, force: &Force, drag_coefficient: f32) -> f32 {
    ((2.0 * mass * force.linear_force.length())
        / (FLUID_DENSITY * CROSS_SECTIONAL_AREA * drag_coefficient))
        .sqrt()
}
