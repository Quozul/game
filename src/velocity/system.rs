use crate::velocity::components::{DragCoefficient, Force, Mass, Velocity};
use bevy::prelude::*;

const FLUID_DENSITY: f32 = 0.05; // 1.2 kg/m^3 at room temperature
const CROSS_SECTIONAL_AREA: f32 = 1.0;

pub fn update_velocity(
    mut q_velocity: Query<(
        &mut Transform,
        &mut Velocity,
        &Force,
        &Mass,
        &DragCoefficient,
    )>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (mut transform, mut velocity, applied_force, mass, drag_coefficient) in
        q_velocity.iter_mut()
    {
        let air_resistance = -get_drag_force(velocity.0, drag_coefficient.0);
        let acceleration = (applied_force.0 / mass.0) + (air_resistance / mass.0);
        velocity.0 += acceleration * delta_time;
        transform.translation += velocity.0.extend(0.) * delta_time;
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
    ((2.0 * mass * force.0.length()) / (FLUID_DENSITY * CROSS_SECTIONAL_AREA * drag_coefficient))
        .sqrt()
}
