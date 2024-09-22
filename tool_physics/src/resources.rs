use bevy::prelude::Resource;

#[derive(Resource)]
pub struct PhysicsResource {
    pub air_density: f32,
    pub newton_gravity: f32,
}

impl Default for PhysicsResource {
    fn default() -> Self {
        Self {
            air_density: 1.2, // 1.2 kg/m^3 at room temperature
            newton_gravity: 9.81,
        }
    }
}
