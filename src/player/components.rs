use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};
use bevy::prelude::*;

pub enum Direction {
	UP,
	DOWN,
	LEFT,
	RIGHT,
}

impl Direction {
	pub fn to_vec(&self) -> Vec2 {
		match self {
			Direction::UP => Vec2::new(0.0, 1.0),
			Direction::DOWN => Vec2::new(0.0, -1.0),
			Direction::LEFT => Vec2::new(-1.0, 0.0),
			Direction::RIGHT => Vec2::new(1.0, 0.0),
		}
	}

	pub fn from_vec(vec: Vec2) -> Direction {
		let angle = vec.y.atan2(vec.x);

		if angle >= -FRAC_PI_4 && angle < FRAC_PI_4 {
			Direction::RIGHT
		} else if angle >= FRAC_PI_4 && angle < FRAC_PI_2 {
			Direction::DOWN
		} else if angle >= FRAC_PI_2 || angle < -FRAC_PI_2 {
			Direction::LEFT
		} else {
			Direction::UP
		}
	}
}

#[derive(PartialEq)]
pub enum PlayerAnimation {
	ATTACKING,
	MOVING,
	IDLING,
	DYING,
}

#[derive(Component)]
pub struct Player {
	pub direction: Direction,
	pub state: PlayerAnimation,
}

#[derive(Component)]
pub struct AttackTimer(pub Timer);

#[derive(Component)]
pub struct DyingTimer(pub Timer);

#[derive(Component)]
pub struct Attack;

#[derive(Bundle)]
pub struct PlayerBundle {
	pub player: Player,
	pub timer: AttackTimer,
}

impl Default for PlayerBundle {
	fn default() -> PlayerBundle {
		PlayerBundle {
			player: Player::default(),
			timer: AttackTimer(Timer::default()),
		}
	}
}

impl Default for Player {
	fn default() -> Player {
		Player {
			direction: Direction::DOWN,
			state: PlayerAnimation::IDLING,
		}
	}
}
