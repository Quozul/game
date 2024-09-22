use crate::components::{DragCoefficient, Force, Impulse, Mass, Sensor, Velocity};
use crate::constants::CROSS_SECTIONAL_AREA;
use crate::resources::PhysicsResource;
use bevy::prelude::*;

/// Applies the force and drag on objects
pub fn apply_acceleration_and_drag(
    physics_resource: Res<PhysicsResource>,
    mut q_velocities: Query<(&mut Velocity, &Force, &DragCoefficient, &Mass)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (mut velocity, force, drag_coefficient, mass) in q_velocities.iter_mut() {
        let air_resistance = get_drag_force(
            physics_resource.air_density,
            velocity.linear_velocity,
            drag_coefficient.0,
        );
        let acceleration = force.linear_force * mass.0;
        velocity.linear_velocity += (acceleration - air_resistance) / mass.0 * delta_time;
    }
}

fn get_drag_force(fluid_density: f32, linear_velocity: Vec2, drag_coefficient: f32) -> Vec2 {
    0.5 * fluid_density
        * linear_velocity.normalize_or_zero()
        * linear_velocity.length_squared()
        * drag_coefficient
        * CROSS_SECTIONAL_AREA
}

/// Applies an impulse to the velocity then resets the impulse to zero
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

/// Applies Newton's law of universal gravitation
pub fn update_gravity(
    physics_resource: Res<PhysicsResource>,
    mut q_velocities: Query<(&Transform, &Mass, Option<&mut Velocity>), Without<Sensor>>,
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
        // FIXME: if the distance is zero, it causes everything to break
        if distance_sq == 0.0 {
            continue;
        }

        let f = physics_resource.newton_gravity / distance_sq;
        let force_unit_mass = delta * f * delta_time;

        if let Some(velocity) = velocity.as_mut() {
            velocity.linear_velocity += force_unit_mass * other_mass.0;
        }
        if let Some(other_velocity) = other_velocity.as_mut() {
            other_velocity.linear_velocity -= force_unit_mass * mass.0;
        }
    }
}

/// Updates the position of each objects given their velocity
pub fn move_object(mut q_velocities: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    let delta_time = time.delta_seconds();

    for (mut transform, velocity) in q_velocities.iter_mut() {
        transform.translation += velocity.linear_velocity.extend(0.) * delta_time;
        transform.rotate_z(velocity.angular_velocity * delta_time);
    }
}
