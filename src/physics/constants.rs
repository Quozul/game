pub const FLUID_DENSITY: f32 = 0.05; // 1.2 kg/m^3 at room temperature
pub const CROSS_SECTIONAL_AREA: f32 = 1.0;
// The original value is 6.674*10E11 m3⋅kg−1⋅s−2
// We adjusted it to 10E0 so that the smallest object that will be attracted is 1 unit in mass
pub const GRAVITY_CONSTANT: f32 = 6.674 * 10E0;
