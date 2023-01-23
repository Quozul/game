use crate::player::components::{Direction, Player, PlayerAnimation, AttackTimer, Attack};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

pub fn animation_update(keys: &Res<Input<KeyCode>>, player: &mut Player, force: bool) {
	let just_released =
		keys.any_just_released(vec![KeyCode::Z, KeyCode::S, KeyCode::D, KeyCode::Q]);

	let animation_update = if just_released || force {
		if keys.pressed(KeyCode::Z) {
			Ok(Some(Direction::UP))
		} else if keys.pressed(KeyCode::S) {
			Ok(Some(Direction::DOWN))
		} else if keys.pressed(KeyCode::D) {
			Ok(Some(Direction::RIGHT))
		} else if keys.pressed(KeyCode::Q) {
			Ok(Some(Direction::LEFT))
		} else {
			Err(Some(PlayerAnimation::IDLING))
		}
	} else if keys.just_pressed(KeyCode::Z) {
		Ok(Some(Direction::UP))
	} else if keys.just_pressed(KeyCode::S) {
		Ok(Some(Direction::DOWN))
	} else if keys.just_pressed(KeyCode::D) {
		Ok(Some(Direction::RIGHT))
	} else if keys.just_pressed(KeyCode::Q) {
		Ok(Some(Direction::LEFT))
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

pub fn movements(mut player_query: Query<(&Player, &mut Velocity)>) {
	for (player, mut velocity) in &mut player_query {
		velocity.linvel = if player.state == PlayerAnimation::MOVING {
			match player.direction {
				Direction::UP => Vec2::new(0.0, 64.0),
				Direction::DOWN => Vec2::new(0.0, -64.0),
				Direction::LEFT => Vec2::new(-64.0, 0.0),
				Direction::RIGHT => Vec2::new(64.0, 0.0),
			}
		} else {
			Vec2::ZERO
		}
	}
}
