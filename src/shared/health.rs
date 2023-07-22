use bevy::prelude::{
    Changed, Commands, Component, DespawnRecursiveExt, Entity, Query, Res, ResMut, Time, Timer,
    TimerMode, Transform, Vec2,
};
use bevy_quinnet::server::Server;
use bevy_rapier2d::prelude::{ExternalImpulse, QueryFilter, RapierContext};
use rand::Rng;
use std::f32::consts::PI;

use crate::direction::{Direction, Facing, Move};
use crate::messages::ServerMessage;
use crate::server_entities::NetworkServerEntity;
use crate::slime_bundle::Slime;

#[derive(Component)]
pub struct Health {
    pub health: u8,
}

pub fn attack_enemies(
    rapier_context: Res<RapierContext>,
    player_query: Query<(Entity, &Transform, &Move, &Facing), Changed<Move>>,
    mut enemy_query: Query<(&mut ExternalImpulse, &mut Health)>,
) {
    for (entity, transform, move_component, facing) in &player_query {
        if move_component.direction.is_attacking() {
            let ray_pos = Vec2::new(transform.translation.x, transform.translation.y);
            let ray_dir = facing.to_vec();
            let max_toi = 50.0;
            let solid = true;
            let filter = QueryFilter::exclude_fixed().exclude_collider(entity);

            if let Some((entity, intersection)) =
                rapier_context.cast_ray_and_get_normal(ray_pos, ray_dir, max_toi, solid, filter)
            {
                if let Ok((mut impulse, mut health)) = enemy_query.get_mut(entity) {
                    let hit_normal = intersection.normal;

                    if health.health > 0 {
                        health.health -= 1;
                        impulse.impulse = hit_normal * -100.0;
                    }
                };
            }
        }
    }
}

pub(crate) fn slime_attack(
    time: Res<Time>,
    mut query: Query<(&mut Move, &mut Facing, &mut Slime)>,
) {
    let mut rng = rand::thread_rng();

    for (mut move_component, mut facing, mut slime) in &mut query {
        if move_component.direction != Direction::Dying {
            slime.last_attack.tick(time.delta());

            if slime.last_attack.finished() {
                slime.last_attack.reset();
                move_component.direction = rand::random();
                facing.angle = rng.gen_range(-PI..PI);
            }
        }
    }
}

#[derive(Component)]
pub struct DeadState {
    pub(crate) elapsed: Timer,
}

pub fn timer_from_frame_count(frame_count: u8) -> Timer {
    Timer::from_seconds(1.0 / 10.0 * frame_count as f32, TimerMode::Once)
}

pub fn animate_dead(mut query: Query<(&Health, &mut Move, &mut DeadState), Changed<Health>>) {
    for (health, mut move_component, mut dead_state) in &mut query {
        if health.health <= 0 {
            move_component.direction = Direction::Dying;
            dead_state.elapsed.reset()
        }
    }
}

pub fn tick_dead(time: Res<Time>, mut query: Query<(&mut DeadState, &Move)>) {
    for (mut dead_animation, move_component) in &mut query {
        if move_component.direction == Direction::Dying {
            dead_animation.elapsed.tick(time.delta());
        }
    }
}

pub fn despawn_dead(
    mut commands: Commands,
    mut server: ResMut<Server>,
    query: Query<(Entity, &Health, &DeadState, &NetworkServerEntity)>,
) {
    for (entity, health, dead_state, server_entity) in &query {
        if health.health <= 0 && dead_state.elapsed.finished() {
            commands.entity(entity).despawn_recursive();

            if let Some(endpoint) = server.get_endpoint_mut() {
                endpoint.try_broadcast_message(ServerMessage::Despawn {
                    id: server_entity.id,
                });
            }
        }
    }
}
