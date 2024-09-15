use crate::velocity::system::update_velocity;
use bevy::prelude::*;

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_velocity);
    }
}
