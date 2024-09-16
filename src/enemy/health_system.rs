use crate::enemy::bundle::EnemyBundle;
use crate::enemy::components::Health;
use crate::physics::colliders::circle_collider::CircleCollider;
use bevy::prelude::*;

pub fn despawn_dead(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_health: Query<(&Transform, &Health, &CircleCollider, Entity)>,
) {
    for (transform, health, circle, entity) in q_health.iter() {
        if health.0 == 0 {
            commands.entity(entity).despawn();
            let new_radius = circle.radius / 2.0;
            if new_radius > 10.0 {
                commands.spawn(EnemyBundle::new(
                    &mut meshes,
                    &mut materials,
                    new_radius,
                    transform.translation,
                ));
            }
        }
    }
}
