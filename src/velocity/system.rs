use crate::velocity::components::{Force, Mass, Velocity};
use bevy::prelude::*;

const FLUID_DENSITY: f32 = 1.2; // TODO: 1.2 kg/m^3 at room temperature
const DRAG_COEFFICIENT: f32 = 1.05; // Cube drag coefficient
const CROSS_SECTIONAL_AREA: f32 = 1.0; // TODO: Should be the area of the shape
const FRICTION_COEFFICIENT: f32 = 0.75;

pub fn update_velocity(
    mut q_velocity: Query<(&mut Transform, &mut Velocity, &Force, &Mass)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (mut transform, mut velocity, applied_force, mass) in q_velocity.iter_mut() {
        let damping = velocity.0 * FRICTION_COEFFICIENT;
        let acceleration = (applied_force.0 - damping) / mass.0;
        velocity.0 += acceleration * delta_time;
        transform.translation += velocity.0.extend(0.) * delta_time;
    }
}

fn get_drag_force(linear_velocity: &Vec2) -> Vec2 {
    1. / 2.
        * FLUID_DENSITY
        * linear_velocity.signum()
        * linear_velocity.powf(2.0)
        * DRAG_COEFFICIENT
        * CROSS_SECTIONAL_AREA
}

#[allow(dead_code)]
fn get_terminal_velocity(mass: f32, force: &Force) -> f32 {
    ((2.0 * mass * force.0.length()) / (FLUID_DENSITY * CROSS_SECTIONAL_AREA * DRAG_COEFFICIENT))
        .sqrt()
}
