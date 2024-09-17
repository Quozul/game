use crate::physics::colliders::circle_collider::CircleCollider;
use crate::physics::colliders::collider::{Collider, CollidesWith};
use crate::physics::colliders::polygon_collider::PolygonCollider;
use crate::physics::events::CollisionEvent;
use crate::physics::movements_components::{Mass, Velocity};
use bevy::prelude::*;

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
                other_adjusted_poly_collider.collides_with(&adjusted_circle_collider)
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

pub fn solve_collisions(
    mut event: EventReader<CollisionEvent>,
    mut q_bodies: Query<(&mut Transform, &mut Velocity, &Mass)>,
) {
    for ev in event.read() {
        if let Ok(
            [(mut transform, mut velocity, mass), (mut other_transform, mut other_velocity, other_mass)],
        ) = q_bodies.get_many_mut([ev.first, ev.second])
        {
            let translation_vector = ev.collision.translation_vector() * 0.5;
            transform.translation -= translation_vector.extend(0.0);
            other_transform.translation += translation_vector.extend(0.0);

            let v1i = velocity.linear_velocity;
            let v2i = other_velocity.linear_velocity;
            let m1 = mass.0;
            let m2 = other_mass.0;

            let total_mass = m1 + m2;

            let vf = (m1 * v1i + m2 * v2i) / total_mass;

            velocity.linear_velocity = vf;
            other_velocity.linear_velocity = -vf;
        }
    }
}

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

pub fn draw_world(mut gizmos: Gizmos) {
    gizmos.grid_2d(
        Vec2::ZERO,
        0.,
        UVec2::new(10, 10),
        Vec2::splat(650.),
        Color::WHITE,
    );
}
