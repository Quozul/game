use crate::colliders::circle_collider::CircleCollider;
use crate::colliders::polygon_collider::PolygonCollider;
use crate::colliders::traits::{Collider, CollidesWith};
use crate::collisions::shape::Shape;
use crate::events::CollisionEvent;
use bevy::prelude::*;

type Data<'a> = (
    &'a Transform,
    Option<&'a PolygonCollider>,
    Option<&'a CircleCollider>,
    Entity,
);
type Filter = Or<(With<PolygonCollider>, With<CircleCollider>)>;

/// Dispatches events when a collision is detected between two objects
pub fn resolve_collisions(mut events: EventWriter<CollisionEvent>, q_bodies: Query<Data, Filter>) {
    let mut iter = q_bodies.iter_combinations();
    while let Some([object_a, object_b]) = iter.fetch_next() {
        let result = Shape::new(object_a)
            .zip(Shape::new(object_b))
            .and_then(|(first, second)| first.collides_with(&second));

        if let Some(collision) = result {
            events.send(CollisionEvent {
                first: object_a.3,
                second: object_b.3,
                collision,
            });
        }
    }
}

/// Debug system to draw the colliders boxes
pub fn draw_colliders(
    mut gizmos: Gizmos,
    q_bodies: Query<(
        &Transform,
        Option<&PolygonCollider>,
        Option<&CircleCollider>,
    )>,
) {
    for (transform, polygon, circle) in q_bodies.iter() {
        if let Some(poly) = polygon {
            poly.transform(transform).draw_collider(&mut gizmos);
        }
        if let Some(circle) = circle {
            circle.transform(transform).draw_collider(&mut gizmos);
        }
    }
}
