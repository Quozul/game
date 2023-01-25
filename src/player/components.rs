use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};
use bevy::prelude::*;

pub enum FacingDirection {
	Up,
	Down,
	Left,
	Right,
}

impl FacingDirection {
	pub fn to_vec(&self) -> Vec2 {
		match self {
			FacingDirection::Up => Vec2::new(0.0, 1.0),
			FacingDirection::Down => Vec2::new(0.0, -1.0),
			FacingDirection::Left => Vec2::new(-1.0, 0.0),
			FacingDirection::Right => Vec2::new(1.0, 0.0),
		}
	}

	pub fn from_vec(vec: Vec2) -> FacingDirection {
		let angle = vec.y.atan2(vec.x);

		if (-FRAC_PI_4..FRAC_PI_4).contains(&angle) {
			FacingDirection::Right
		} else if (FRAC_PI_4..FRAC_PI_2).contains(&angle) {
			FacingDirection::Down
		} else if !(-FRAC_PI_2..FRAC_PI_2).contains(&angle) {
			FacingDirection::Left
		} else {
			FacingDirection::Up
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
	pub direction: FacingDirection,
	pub state: PlayerAnimation,
}

#[derive(Component)]
pub struct FacingComponent(pub(crate) FacingDirection);

impl Default for FacingComponent {
	fn default() -> Self {
		FacingComponent(FacingDirection::Down)
	}
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
			direction: FacingDirection::Down,
			state: PlayerAnimation::IDLING,
		}
	}
}
