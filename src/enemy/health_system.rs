use crate::enemy::bundle::EnemyBundle;
use crate::enemy::components::Health;
use crate::physics::colliders::polygon_collider::PolygonCollider;
use bevy::prelude::*;

pub fn despawn_dead(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_health: Query<(&Transform, &Health, &PolygonCollider, Entity)>,
) {
    for (transform, health, asteroid_collider, entity) in q_health.iter() {
        if health.0 == 0 {
            commands.entity(entity).despawn();
            let new_radius = asteroid_collider.get_radius() / 2.0;
            let new_polygon_count = asteroid_collider.vertices_count() - 1;

            if new_polygon_count >= 3 && new_radius >= 10.0 {
                commands.spawn(EnemyBundle::new(
                    &mut meshes,
                    &mut materials,
                    new_radius,
                    new_polygon_count,
                    transform.translation,
                    Vec2::ZERO,
                ));
            }
        }
    }
}
