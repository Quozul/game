use crate::interact::components::Interactive;
use bevy::prelude::Bundle;
use tool_physics::circle_collider::CircleCollider;
use tool_physics::Sensor;

#[derive(Bundle)]

pub struct InteractBundle {
    pub interactive: Interactive,
    pub collider: CircleCollider,
    pub sensor: Sensor,
}

impl InteractBundle {
    const INTERACT_RADIUS: f32 = 50.0;

    pub fn new() -> Self {
        Self {
            interactive: Interactive,
            collider: CircleCollider::circle(Self::INTERACT_RADIUS),
            sensor: Sensor,
        }
    }
}
