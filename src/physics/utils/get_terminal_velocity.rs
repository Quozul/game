use crate::physics::constants::CROSS_SECTIONAL_AREA;

pub fn get_terminal_velocity(fluid_density: f32, force: f32, drag_coefficient: f32) -> f32 {
    ((2.0 * force) / (fluid_density * CROSS_SECTIONAL_AREA * drag_coefficient)).sqrt()
}
