use crate::physics::collisions_systems::{draw_world, resolve_collision};
use crate::physics::movements_systems::{
    apply_acceleration_and_drag, move_object, update_gravity, update_impulse,
};
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_world).add_systems(
            PostUpdate,
            (
                update_impulse,
                update_gravity,
                apply_acceleration_and_drag,
                move_object,
                resolve_collision,
            ),
        );
    }
}
