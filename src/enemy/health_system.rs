use crate::animation::components::AnimationConfig;
use crate::enemy::components::{Health, UiArrow};
use crate::physics::colliders::circle_collider::CircleCollider;
use bevy::prelude::*;

pub fn start_dead_animation(
    mut commands: Commands,
    mut q_health: Query<(&Health, &UiArrow, &mut AnimationConfig, Entity), Changed<Health>>,
) {
    for (health, arrow, mut animation, entity) in q_health.iter_mut() {
        if health.0 == 0 {
            commands.entity(entity).remove::<CircleCollider>();
            if let Some(mut ent) = commands.get_entity(arrow.0) {
                ent.despawn();
                animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
            }
        }
    }
}

pub fn despawn_dead(mut commands: Commands, q_health: Query<(&Health, &AnimationConfig, Entity)>) {
    for (health, animation, entity) in q_health.iter() {
        if health.0 == 0 && animation.frame_timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
