use bevy::prelude::Resource;

#[derive(Resource)]
pub struct PhysicsResource {
    pub air_density: f32,
    pub newton_gravity: f32,
}
