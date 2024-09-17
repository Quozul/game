use crate::physics::colliders::circle_collider::CircleCollider;
use crate::physics::colliders::collider::{Collider, CollidesWith};
use crate::physics::colliders::polygon_collider::PolygonCollider;
use crate::physics::events::CollisionEvent;
use crate::physics::movements_components::{Mass, RigidBodyType, Velocity};
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
    mut q_bodies: Query<(&Transform, Option<&mut Velocity>, Option<&Mass>)>,
) {
    for ev in event.read() {
        if let Ok([(transform_a, velocity_a, mass_a), (transform_b, velocity_b, mass_b)]) =
            q_bodies.get_many_mut([ev.first, ev.second])
        {
            let va = if let Some(ref velocity_a) = velocity_a {
                velocity_a.linear_velocity
            } else {
                Vec2::ZERO
            };

            let vb = if let Some(ref velocity_b) = velocity_b {
                velocity_b.linear_velocity
            } else {
                Vec2::ZERO
            };

            let obj1 = Object {
                mass: mass_a.map(|m| m.0).unwrap_or(1.0),
                velocity: va,
                position: transform_a.translation.xy(),
            };
            let obj2 = Object {
                mass: mass_b.map(|m| m.0).unwrap_or(1.0),
                velocity: vb,
                position: transform_b.translation.xy(),
            };
            let (velocity_final_a, velocity_final_b) =
                restitute_energy_after_impact(obj1, obj2, 0.5);

            if let Some(mut velocity_a) = velocity_a {
                velocity_a.linear_velocity = velocity_final_a;
            }
            if let Some(mut velocity_b) = velocity_b {
                velocity_b.linear_velocity = velocity_final_b;
            }
        }
    }
}

struct Object {
    mass: f32,
    velocity: Vec2,
    position: Vec2,
}

fn restitute_energy_after_impact(obj1: Object, obj2: Object, e: f32) -> (Vec2, Vec2) {
    let v_rel = obj1.velocity - obj2.velocity;
    let n = (obj1.position - obj2.position).normalize();
    let t = n.perp();

    let v_rel_n = v_rel.dot(n);
    let v_rel_t = v_rel.dot(t);

    let v_rel_n_prime = -e * v_rel_n;
    let v_rel_prime = v_rel_n_prime * n + v_rel_t * t;

    let new_velocity1 =
        obj1.velocity - (obj2.mass / (obj1.mass + obj2.mass)) * (v_rel - v_rel_prime);
    let new_velocity2 =
        obj2.velocity + (obj1.mass / (obj1.mass + obj2.mass)) * (v_rel - v_rel_prime);

    (new_velocity1, new_velocity2)
}

pub fn solve_collisions_transforms(
    mut event: EventReader<CollisionEvent>,
    mut q_bodies: Query<(&mut Transform, &RigidBodyType)>,
) {
    for ev in event.read() {
        if let Ok(
            [(mut transform, rigid_body_type), (mut other_transform, other_rigid_body_type)],
        ) = q_bodies.get_many_mut([ev.first, ev.second])
        {
            let translation_vector = (ev.collision.translation_vector() * 0.5).extend(0.0);
            if *rigid_body_type == RigidBodyType::Dynamic {
                transform.translation -= translation_vector;
            }
            if *other_rigid_body_type == RigidBodyType::Dynamic {
                other_transform.translation += translation_vector;
            }
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
