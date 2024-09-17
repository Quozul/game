use crate::physics::collisions::collisions_systems::{
    draw_colliders, draw_world, resolve_collisions,
};
use crate::physics::collisions::solver::{
    solve_collisions_transforms, solve_collisions_velocities,
};
use crate::physics::dynamics::systems::{
    apply_acceleration_and_drag, move_object, update_gravity, update_impulse,
};
use crate::physics::events::CollisionEvent;
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(Update, (draw_world, draw_colliders))
            .add_systems(
                PostUpdate,
                (
                    (
                        update_impulse,
                        update_gravity,
                        apply_acceleration_and_drag,
                        move_object,
                    ),
                    resolve_collisions,
                    solve_collisions_velocities,
                    solve_collisions_transforms,
                )
                    .chain(),
            );
    }
}
