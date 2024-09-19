use crate::physics::colliders::circle_collider::CircleCollider;
use crate::physics::components::Sensor;
use bevy::prelude::*;

#[derive(Component)]
pub struct Item<T>(pub T);

#[derive(Bundle)]
pub struct ItemBundle<T: Send + Sync + 'static> {
    collider: CircleCollider,
    sensor: Sensor,
    item: Item<T>,
}

impl<T: Send + Sync> ItemBundle<T> {
    pub fn new(item: T) -> Self {
        Self {
            collider: CircleCollider::circle(50.0),
            sensor: Sensor,
            item: Item(item),
        }
    }
}
