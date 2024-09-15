use crate::physics::components::{DragCoefficient, Force, Impulse, Mass, Velocity};
use crate::physics::constants::{CROSS_SECTIONAL_AREA, FLUID_DENSITY, GRAVITY_CONSTANT};
use bevy::prelude::*;

pub fn apply_acceleration_and_drag(
    mut q_velocities: Query<(&mut Velocity, &Force, &DragCoefficient, &Mass)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (mut velocity, applied_force, drag_coefficient, mass) in q_velocities.iter_mut() {
        let air_resistance = get_drag_force(velocity.linear_velocity, drag_coefficient.0);
        let acceleration = applied_force.linear_force;
        velocity.linear_velocity += (acceleration - air_resistance) / mass.0 * delta_time;
    }
}

/// Applies an impulse to the velocity then resets the impulse
pub fn update_impulse(
    mut q_velocities: Query<(&mut Velocity, &mut Impulse), Changed<Impulse>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (mut velocity, mut impulse) in q_velocities.iter_mut() {
        if impulse.linear_impulse.x != 0.0 || impulse.linear_impulse.y != 0.0 {
            velocity.linear_velocity += impulse.linear_impulse * delta_time;
            impulse.linear_impulse = Vec2::ZERO;
        }
    }
}

/// Apply Newton's law of universal gravitation
pub fn update_gravity(
    mut q_velocities: Query<(&Transform, &Mass, &mut Velocity)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    let mut iter = q_velocities.iter_combinations_mut();
    while let Some(
        [(transform, mass, mut velocity), (other_transform, other_mass, mut other_velocity)],
    ) = iter.fetch_next()
    {
        let delta = (other_transform.translation - transform.translation).xy();
        let distance_sq = delta.length_squared();
        let f = GRAVITY_CONSTANT / distance_sq;
        let force_unit_mass = delta * f * delta_time;

        velocity.linear_velocity += force_unit_mass * other_mass.0;
        other_velocity.linear_velocity -= force_unit_mass * mass.0;
    }
}

pub fn move_object(mut q_velocities: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    let delta_time = time.delta_seconds();

    for (mut transform, velocity) in q_velocities.iter_mut() {
        transform.translation += velocity.linear_velocity.extend(0.) * delta_time;
    }
}

fn get_drag_force(linear_velocity: Vec2, drag_coefficient: f32) -> Vec2 {
    0.5 * FLUID_DENSITY
        * linear_velocity.normalize_or_zero()
        * linear_velocity.length_squared()
        * drag_coefficient
        * CROSS_SECTIONAL_AREA
}
