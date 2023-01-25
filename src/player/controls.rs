use crate::player::components::{FacingDirection, Player, PlayerAnimation, AttackTimer, Attack};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

pub fn animation_update(keys: &Res<Input<KeyCode>>, player: &mut Player, force: bool) {
	let just_released =
		keys.any_just_released(vec![KeyCode::Z, KeyCode::S, KeyCode::D, KeyCode::Q]);

	let animation_update = if just_released || force {
		if keys.pressed(KeyCode::Z) {
			Ok(Some(FacingDirection::Up))
		} else if keys.pressed(KeyCode::S) {
			Ok(Some(FacingDirection::Down))
		} else if keys.pressed(KeyCode::D) {
			Ok(Some(FacingDirection::Right))
		} else if keys.pressed(KeyCode::Q) {
			Ok(Some(FacingDirection::Left))
		} else {
			Err(Some(PlayerAnimation::IDLING))
		}
	} else if keys.just_pressed(KeyCode::Z) {
		Ok(Some(FacingDirection::Up))
	} else if keys.just_pressed(KeyCode::S) {
		Ok(Some(FacingDirection::Down))
	} else if keys.just_pressed(KeyCode::D) {
		Ok(Some(FacingDirection::Right))
	} else if keys.just_pressed(KeyCode::Q) {
		Ok(Some(FacingDirection::Left))
	} else {
		Err(None)
	};

	match animation_update {
		Ok(ok) => {
			if let Some(direction) = ok {
				player.direction = direction;
				player.state = PlayerAnimation::MOVING;
			}
		}
		Err(err) => {
			if let Some(state) = err {
				player.state = state;
			}
		}
	};
}

pub fn controls(
	mut commands: Commands,
	keys: Res<Input<KeyCode>>,
	mut player_query: Query<(&mut Player, &mut AttackTimer, Entity)>,
) {
	for (mut player, mut timer, entity) in &mut player_query {
		if player.state != PlayerAnimation::ATTACKING {
			animation_update(&keys, player.as_mut(), false);

			if keys.just_pressed(KeyCode::Space) {
				commands.entity(entity).insert(Attack);
				player.state = PlayerAnimation::ATTACKING;
				timer.0 = Timer::from_seconds(1. / 10. * 4., TimerMode::Once)
			};
		}
	}
}
