use crate::physics::collisions::collision::Collision;
use bevy::prelude::*;

pub trait Collider {
    fn draw_collider(&self, gizmos: &mut Gizmos);
    fn transform(&self, transform: &Transform) -> Self;
}

pub trait CollidesWith<T> {
    fn collides_with(&self, other: &T) -> Option<Collision>;
}
