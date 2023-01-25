use crate::animations::AnimationState;
use crate::player::components::{Attack, FacingDirection, DyingTimer, Player, PlayerAnimation};
use crate::slimes::{Health, Slime};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn timer_from_frame_count(frame_count: u8) -> Timer {
	Timer::from_seconds(1.0 / 10.0 * frame_count as f32, TimerMode::Once)
}

pub fn attack_enemies(
	mut commands: Commands,
	rapier_context: Res<RapierContext>,
	player_query: Query<(&Player, &Transform, Entity), (With<Attack>, Without<Slime>)>,
	mut slime_query: Query<(&mut Player, &mut AnimationState, &mut ExternalImpulse, &mut Health), With<Slime>>,
) {
	for (player, transform, entity) in &player_query {
		commands.entity(entity).remove::<Attack>();

		let ray_pos = Vec2::new(transform.translation.x, transform.translation.y);
		let ray_dir = player.direction.to_vec();
		let max_toi = 20.0;
		let solid = true;
		let filter = QueryFilter::exclude_fixed().exclude_collider(entity);

		if let Some((entity, intersection)) = rapier_context.cast_ray_and_get_normal(
			ray_pos, ray_dir, max_toi, solid, filter
		) {
			if let Ok((mut slime, mut state, mut impulse, mut health)) = slime_query.get_mut(entity) {
				let hit_normal = intersection.normal;

				if health.0 > 0 {
					health.0 -= 1;
					impulse.impulse = hit_normal * -100.0;
					slime.direction = FacingDirection::from_vec(hit_normal);

					if health.0 == 0 {
						slime.state = PlayerAnimation::DYING;
						state.0.reset();
						commands.entity(entity).insert(DyingTimer(timer_from_frame_count(5)));
					}
				}
			};
		}
	}
}

/// Despawns the entity after it dies.
pub fn dying_animation(
	time: Res<Time>,
	mut commands: Commands,
	mut query: Query<(&mut DyingTimer, Entity)>,
) {
	for (mut timer, entity) in &mut query {
		timer.0.tick(time.delta());
		if timer.0.finished() {
			commands.entity(entity).despawn_recursive();
		}
	}
}
