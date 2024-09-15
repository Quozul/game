use crate::physics::systems::{
    apply_acceleration_and_drag, move_object, update_gravity, update_impulse,
};
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                update_impulse,
                update_gravity,
                apply_acceleration_and_drag,
                move_object,
            ),
        );
    }
}
