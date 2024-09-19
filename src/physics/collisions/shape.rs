use crate::physics::colliders::circle_collider::CircleCollider;
use crate::physics::colliders::polygon_collider::PolygonCollider;
use crate::physics::colliders::traits::{Collider, CollidesWith};
use crate::physics::collisions::collision::Collision;
use bevy::prelude::{Entity, Transform};

pub enum Shape {
    Polygon(PolygonCollider),
    Circle(CircleCollider),
}

impl CollidesWith<Shape> for Shape {
    fn collides_with(&self, other: &Shape) -> Option<Collision> {
        match (self, other) {
            (Shape::Polygon(a), Shape::Polygon(b)) => a.collides_with(b),
            (Shape::Polygon(a), Shape::Circle(b)) => a.collides_with(b),
            (Shape::Circle(a), Shape::Polygon(b)) => a.collides_with(b),
            (Shape::Circle(a), Shape::Circle(b)) => a.collides_with(b),
        }
    }
}

impl Shape {
    pub fn new(
        pair: (
            &Transform,
            Option<&PolygonCollider>,
            Option<&CircleCollider>,
            Entity,
        ),
    ) -> Option<Self> {
        let transform = pair.0;
        let polygon = pair.1;
        let circle = pair.2;
        polygon
            .map(|collider| Shape::Polygon(collider.transform(transform)))
            .or_else(|| circle.map(|collider| Shape::Circle(collider.transform(transform))))
    }
}
