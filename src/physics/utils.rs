use crate::physics::constants::{CROSS_SECTIONAL_AREA, FLUID_DENSITY};

pub fn get_terminal_velocity(mass: f32, force: f32, drag_coefficient: f32) -> f32 {
    ((2.0 * /*mass **/ force) / (FLUID_DENSITY * CROSS_SECTIONAL_AREA * drag_coefficient)).sqrt()
}
