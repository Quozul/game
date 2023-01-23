use crate::animations::AnimationState;
use crate::player::components::{Direction, Player, PlayerAnimation};
use crate::slimes::Slime;
use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::mem::align_of;

pub fn attack_enemies(
	rapier_context: Res<RapierContext>,
	player_query: Query<(&Player, &Transform, Entity), Without<Slime>>,
	mut slime_query: Query<(&mut Player, &mut AnimationState), With<Slime>>,
) {
	for (player, transform, entity) in &player_query {
		if player.state == PlayerAnimation::ATTACKING {
			let ray_pos = Vec2::new(transform.translation.x, transform.translation.y);
			let ray_dir = match player.direction {
				Direction::UP => Vec2::new(0.0, 1.0),
				Direction::DOWN => Vec2::new(0.0, -1.0),
				Direction::LEFT => Vec2::new(-1.0, 0.0),
				Direction::RIGHT => Vec2::new(1.0, 0.0),
			};
			let max_toi = 20.0;
			let solid = true;
			let filter = QueryFilter::exclude_fixed().exclude_collider(entity);

			if let Some((entity, toi)) =
				rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
			{
				// The first collider hit has the entity `entity` and it hit after
				// the ray travelled a distance equal to `ray_dir * toi`.
				if let Ok((mut slime, mut state)) = slime_query.get_mut(entity) {
					if slime.state != PlayerAnimation::DYING {
						slime.state = PlayerAnimation::DYING;
						state.0.reset();
					}
				};
			}
		}
	}
}
