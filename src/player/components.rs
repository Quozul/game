use bevy::prelude::*;

pub enum Direction {
	UP,
	DOWN,
	LEFT,
	RIGHT,
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
pub struct PlayerTimer(pub(crate) Timer);

#[derive(Bundle)]
pub struct PlayerBundle {
	pub player: Player,
	pub timer: PlayerTimer,
}

impl Default for PlayerBundle {
	fn default() -> PlayerBundle {
		PlayerBundle {
			player: Player::default(),
			timer: PlayerTimer(Timer::default()),
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
