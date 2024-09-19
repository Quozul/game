use crate::physics::colliders::circle_collider::CircleCollider;
use crate::physics::components::Sensor;
use bevy::prelude::*;

#[derive(Component)]
pub struct Item<T>(pub T);

#[derive(Bundle)]
pub struct ItemBundle<T: Send + Sync + 'static> {
    transform: Transform,
    collider: CircleCollider,
    sensor: Sensor,
    item: Item<T>,
}

impl<T: Send + Sync> ItemBundle<T> {
    pub fn new(translation: Vec2, item: T) -> Self {
        Self {
            transform: Transform::from_translation(translation.extend(0.0)),
            collider: CircleCollider::circle(50.0),
            sensor: Sensor,
            item: Item(item),
        }
    }
}
