use bevy::prelude::{Changed, Component, Entity, Query, Res, Transform, Vec2};
use bevy_rapier2d::prelude::{ExternalImpulse, QueryFilter, RapierContext};

use crate::direction::Move;

#[derive(Component)]
pub struct Health {
    pub(crate) health: u8,
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
            let max_toi = 20.0;
            let solid = true;
            let filter = QueryFilter::exclude_fixed().exclude_collider(entity);

            if let Some((entity, intersection)) =
                rapier_context.cast_ray_and_get_normal(ray_pos, ray_dir, max_toi, solid, filter)
            {
                println!("Found {:?}", entity);

                if let Ok((mut impulse, mut health)) = enemy_query.get_mut(entity) {
                    let hit_normal = intersection.normal;

                    if health.health > 0 {
                        health.health -= 1;
                        impulse.impulse = hit_normal * -100.0;

                        // TODO: Handle death
                    }
                };
            }
        }
    }
}
