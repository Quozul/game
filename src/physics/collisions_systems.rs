use crate::physics::collision_components::PolygonCollider;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

pub fn resolve_collision(
    mut gizmos: Gizmos,
    mut q_bodies: Query<(&mut Transform, &PolygonCollider)>,
) {
    let mut iter = q_bodies.iter_combinations_mut();
    while let Some([(mut transform, collider), (mut other_transform, other_collider)]) =
        iter.fetch_next()
    {
        let adjusted_collider = adjust_collider_vertices(&transform, collider);
        let other_adjusted_collider = adjust_collider_vertices(&other_transform, other_collider);

        adjusted_collider.draw_collider(&mut gizmos);
        other_adjusted_collider.draw_collider(&mut gizmos);

        if let Some((normal, depth)) = adjusted_collider.intersect_polygon(&other_adjusted_collider)
        {
            // TODO: Update Velocity instead of Transform
            transform.translation -= (normal * depth * 0.5).extend(0.0);
            other_transform.translation += (normal * depth * 0.5).extend(0.0);
            // TODO: Fire collision event with the Entity ID
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

fn adjust_collider_vertices(transform: &Transform, collider: &PolygonCollider) -> PolygonCollider {
    let adjusted_vertices = collider
        .vertices
        .iter()
        .map(|vertex| transform.transform_point(vertex.extend(0.0)).xy())
        .collect::<Vec<_>>();

    PolygonCollider::new(adjusted_vertices)
}
