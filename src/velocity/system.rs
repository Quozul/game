use crate::velocity::components::{Force, Mass, Velocity};
use bevy::prelude::*;

pub fn update_velocity(
    mut q_velocity: Query<(&mut Transform, &mut Velocity, &Force, &Mass)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for (mut transform, mut velocity, applied_force, mass) in q_velocity.iter_mut() {
        let drag_force = get_drag_force(&velocity.0);

        let air_resistance = -drag_force;
        let acceleration = applied_force.0 + air_resistance;
        velocity.0 += (acceleration / mass.0) * delta_time;
        transform.translation += velocity.0.extend(0.);
    }
}

fn get_drag_force(linear_velocity: &Vec2) -> Vec2 {
    let fluid_density = 2.0; // TODO: 1.2 kg/m^3 at room temperature
    let drag_coefficient = 1.05; // Cube drag coefficient
    let cross_sectional_area = 1.0; // TODO: Should be the area of the shape

    1. / 2.
        * fluid_density
        * linear_velocity.signum()
        * linear_velocity.powf(2.0)
        * drag_coefficient
        * cross_sectional_area
}
