use crate::collisions::solver::{solve_collisions_transforms, solve_collisions_velocities};
use crate::collisions::systems::{draw_colliders, resolve_collisions};
use crate::dynamics::schedule::DynamicsSchedule;
use crate::dynamics::systems::{
    apply_acceleration_and_drag, move_object, update_gravity, update_impulse,
};
use crate::events::CollisionEvent;
use crate::resources::PhysicsResource;
use bevy::app::MainScheduleOrder;
use bevy::prelude::*;

pub struct PhysicsPlugin {
    pub draw_debug_colliders: bool,
    pub air_density: f32,
    pub newton_gravity: f32,
}

impl Default for PhysicsPlugin {
    fn default() -> Self {
        Self {
            draw_debug_colliders: false,
            air_density: 1.2, // 1.2 kg/m^3 at room temperature
            newton_gravity: 9.81,
        }
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        if self.draw_debug_colliders {
            app.add_systems(Update, draw_colliders);
        }

        let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();
        main_schedule_order.insert_before(PostUpdate, DynamicsSchedule);

        if self.newton_gravity != 0.0 {
            app.add_systems(DynamicsSchedule, update_gravity);
        }

        app.insert_resource(PhysicsResource {
            air_density: self.air_density,
            newton_gravity: self.newton_gravity,
        })
        .add_event::<CollisionEvent>()
        .add_systems(
            DynamicsSchedule,
            (update_impulse, apply_acceleration_and_drag, move_object),
        )
        .add_systems(
            PostUpdate,
            (
                resolve_collisions,
                solve_collisions_velocities,
                solve_collisions_transforms,
            ),
        );
    }
}
