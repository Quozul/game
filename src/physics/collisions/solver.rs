use crate::physics::components::{Mass, RigidBodyType, Velocity};
use crate::physics::events::CollisionEvent;
use bevy::math::Vec2;
use bevy::prelude::*;

/// Adjusts the positions of colliding bodies so that they don't overlap anymore.
pub fn solve_collisions_transforms(
    mut event: EventReader<CollisionEvent>,
    mut q_bodies: Query<(&mut Transform, &RigidBodyType)>,
) {
    for ev in event.read() {
        if let Ok(
            [(mut transform, rigid_body_type), (mut other_transform, other_rigid_body_type)],
        ) = q_bodies.get_many_mut([ev.first, ev.second])
        {
            // move by half the size of the translation vector if both are dynamic,
            // else move the only dynamic one by the full translation vector
            let ratio = if *rigid_body_type == RigidBodyType::Dynamic
                && *other_rigid_body_type == RigidBodyType::Dynamic
            {
                0.5
            } else {
                1.0
            };

            let translation_vector = (ev.collision.translation_vector() * ratio).extend(0.0);
            if *rigid_body_type == RigidBodyType::Dynamic {
                transform.translation -= translation_vector;
            }
            if *other_rigid_body_type == RigidBodyType::Dynamic {
                other_transform.translation += translation_vector;
            }
        }
    }
}

pub fn solve_collisions_velocities(
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
