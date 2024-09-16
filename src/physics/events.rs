use bevy::prelude::*;

#[derive(Event)]
pub struct CollisionEvent {
    pub first: Entity,
    pub second: Entity,
    pub collision: Vec2,
}
