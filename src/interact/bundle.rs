use crate::interact::components::Interactive;
use crate::physics::colliders::circle_collider::CircleCollider;
use crate::physics::components::Sensor;
use bevy::prelude::Bundle;

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
