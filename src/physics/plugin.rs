use crate::physics::system::{apply_acceleration, apply_drag, move_object, update_impulse};
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_impulse, apply_acceleration, apply_drag, move_object).chain(),
        );
    }
}
