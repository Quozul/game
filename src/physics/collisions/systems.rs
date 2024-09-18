use crate::physics::colliders::circle_collider::CircleCollider;
use crate::physics::colliders::polygon_collider::PolygonCollider;
use crate::physics::colliders::traits::{Collider, CollidesWith};
use crate::physics::events::CollisionEvent;
use bevy::prelude::*;

/// Dispatches events when a collision is detected between two objects
pub fn resolve_collisions(
    mut events: EventWriter<CollisionEvent>,
    q_bodies: Query<(
        &Transform,
        Option<&PolygonCollider>,
        Option<&CircleCollider>,
        Entity,
    )>,
) {
    let mut iter = q_bodies.iter_combinations();
    while let Some(
        [(transform, collider, circle_collider, entity), (other_transform, other_collider, other_circle_collider, other_entity)],
    ) = iter.fetch_next()
    {
        let result = match (
            collider,
            circle_collider,
            other_collider,
            other_circle_collider,
        ) {
            // Polygon-polygon collision
            (Some(poly_collider), None, Some(other_poly_collider), None) => {
                poly_collider.collides_with(other_poly_collider);
                let adjusted_collider = poly_collider.transform(transform);
                let other_adjusted_collider = other_poly_collider.transform(other_transform);
                adjusted_collider.collides_with(&other_adjusted_collider)
            }

            // Polygon-circle collision
            (None, Some(circle_collider), Some(other_poly_collider), None) => {
                let other_adjusted_poly_collider = other_poly_collider.transform(other_transform);
                let adjusted_circle_collider = circle_collider.transform(transform);
                adjusted_circle_collider.collides_with(&other_adjusted_poly_collider)
            }
            (Some(poly_collider), None, None, Some(other_circle_collider)) => {
                let adjusted_poly_collider = poly_collider.transform(transform);
                let other_adjusted_circle_collider =
                    other_circle_collider.transform(other_transform);
                adjusted_poly_collider.collides_with(&other_adjusted_circle_collider)
            }

            // Circle-circle collision
            (None, Some(circle_collider), None, Some(other_circle_collider)) => {
                let adjusted_collider = circle_collider.transform(transform);
                let other_adjusted_collider = other_circle_collider.transform(other_transform);
                adjusted_collider.collides_with(&other_adjusted_collider)
            }

            _ => continue,
        };

        if let Some(collision) = result {
            events.send(CollisionEvent {
                first: entity,
                second: other_entity,
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
