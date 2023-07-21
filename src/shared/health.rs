use bevy::prelude::{
    Changed, Commands, Component, DespawnRecursiveExt, Entity, Query, Res, ResMut, Time, Transform,
    Vec2,
};
use bevy_quinnet::server::Server;
use bevy_rapier2d::prelude::{ExternalImpulse, QueryFilter, RapierContext};

use crate::direction::Move;
use crate::messages::ServerMessage;
use crate::server_entities::NetworkServerEntity;
use crate::slime_bundle::Slime;

#[derive(Component)]
pub struct Health {
    pub health: u8,
}

pub fn attack_enemies(
    rapier_context: Res<RapierContext>,
    player_query: Query<(Entity, &Transform, &Move), Changed<Move>>,
    mut enemy_query: Query<(&mut ExternalImpulse, &mut Health)>,
) {
    for (entity, transform, move_component) in &player_query {
        if move_component.direction.is_attacking() {
            let ray_pos = Vec2::new(transform.translation.x, transform.translation.y);
            let ray_dir = move_component.direction.to_facing_direction().to_vec();
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

pub(crate) fn slime_attack(time: Res<Time>, mut query: Query<(&mut Move, &mut Slime)>) {
    for (mut move_component, mut slime) in &mut query {
        slime.last_attack.tick(time.delta());

        if slime.last_attack.finished() {
            slime.last_attack.reset();
            move_component.direction = rand::random();
        }
    }
}

pub fn despawn_dead(
    mut commands: Commands,
    mut server: ResMut<Server>,
    query: Query<(Entity, &Health, &NetworkServerEntity)>,
) {
    for (entity, health, server_entity) in &query {
        if health.health <= 0 {
            commands.entity(entity).despawn_recursive();

            if let Some(endpoint) = server.get_endpoint_mut() {
                endpoint.try_broadcast_message(ServerMessage::Despawn {
                    id: server_entity.id,
                });
            }
        }
    }
}
