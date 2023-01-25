use crate::animations::*;
use crate::player::components::{FacingDirection, Player, PlayerAnimation, AttackTimer};
use crate::player::controls::animation_update;
use crate::slimes::Slime;
use benimator::FrameRate;
use bevy::prelude::*;

pub fn attack_animation(
	keys: Res<Input<KeyCode>>,
	time: Res<Time>,
	mut player_query: Query<(&mut Player, &mut AttackTimer)>,
) {
	for (mut player, mut player_timer) in &mut player_query {
		if player.state == PlayerAnimation::ATTACKING {
			player_timer.0.tick(time.delta());
			if player_timer.0.finished() {
				animation_update(&keys, player.as_mut(), true)
			}
		}
	}
}

pub fn update_animation(
	mut query: Query<
		(
			&Player,
			&mut Animation,
			&mut AnimationData,
			&mut AnimationState,
		),
		(Changed<Player>, Without<Slime>),
	>,
) {
	for (player, mut animation, mut data, mut state) in &mut query {
		let mut flip_x = false;

		let range = match player.state {
			PlayerAnimation::ATTACKING => {
				state.0.reset();

				match player.direction {
					FacingDirection::Up => 48..=51,
					FacingDirection::Down => 36..=39,
					FacingDirection::Left => {
						flip_x = true;
						42..=45
					}
					FacingDirection::Right => 42..=45,
				}
			}
			PlayerAnimation::MOVING => match player.direction {
				FacingDirection::Up => 30..=33,
				FacingDirection::Down => 18..=23,
				FacingDirection::Left => {
					flip_x = true;
					24..=29
				}
				FacingDirection::Right => 24..=29,
			},
			PlayerAnimation::IDLING => match player.direction {
				FacingDirection::Up => 12..=17,
				FacingDirection::Down => 0..=5,
				FacingDirection::Left => {
					flip_x = true;
					6..=11
				}
				FacingDirection::Right => 6..=11,
			},
			PlayerAnimation::DYING => 48..=50,
		};

		data.flip_x = flip_x;

		animation.0 = benimator::Animation::from_indices(range, FrameRate::from_fps(10.0));
	}
}
