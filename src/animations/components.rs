use benimator::FrameRate;
use bevy::prelude::{Bundle, Component, Timer};
use bevy::time::TimerMode;
use crate::animations::legacy::AnimationTimer;
use crate::player::components::{Direction, Player, PlayerAnimation};

#[derive(Component)]
struct AnimationData {
	animation: benimator::Animation,
}

impl Default for AnimationData {
	fn default() -> Self {
		AnimationData {
			animation: benimator::Animation::from_indices(
				0..=0,
				FrameRate::from_fps(10.0),
			),
		}
	}
}

struct AnimationDirections {
	down: AnimationData,
	left: Option<AnimationData>,
	right: Option<AnimationData>,
	up: Option<AnimationData>,
}

impl AnimationDirections {
	fn from(direction: Direction, indices: impl IntoIterator<Item=usize>) -> AnimationDirections {
		let mut directions = AnimationDirections::default();
		directions.set(direction, indices);
		directions
	}

	fn set(&mut self, direction: Direction, indices: impl IntoIterator<Item=usize>) {
		let animation = benimator::Animation::from_indices(indices, FrameRate::from_fps(10.0));

		match direction {
			Direction::Up => {
				let data = AnimationData {
					animation,
				};
				self.up = Some(data);
			}
			Direction::Down => {
				self.down = AnimationData {
					animation,
				};
			}
			Direction::Left => {
				let data = AnimationData {
					animation,
				};
				self.left = Some(data);
			}
			Direction::Right => {
				let data = AnimationData {
					animation,
				};
				self.right = Some(data);
			}
		}
	}

	fn get_animation(self, direction: Direction) -> &AnimationData {
		let data = match direction {
			Direction::Up => &self.up,
			Direction::Down => &Some(self.down),
			Direction::Left => &self.left,
			Direction::Right => &self.right,
		};

		if let Some(data) = data {
			data
		} else {
			&self.down
		}
	}
}

impl Default for AnimationDirections {
	fn default() -> Self {
		AnimationDirections {
			down: AnimationData::default(),
			left: None,
			right: None,
			up: None,
		}
	}
}

#[derive(Component)]
pub struct AnimationSet {
	idling: AnimationDirections,
	moving: Option<AnimationDirections>,
	attacking: Option<AnimationDirections>,
	dying: Option<AnimationDirections>,
}

impl Default for AnimationSet {
	fn default() -> Self {
		AnimationSet {
			idling: AnimationDirections::default(),
			moving: None,
			attacking: None,
			dying: None,
		}
	}
}

impl AnimationSet {
	fn get_animation(self, animation: PlayerAnimation, direction: Direction) -> AnimationData {
		let data = match animation {
			PlayerAnimation::ATTACKING => {
				self.attacking.get_animation(direction)
			}
			PlayerAnimation::MOVING => {}
			PlayerAnimation::IDLING => {}
			PlayerAnimation::DYING => {}
		}
	}
}

#[derive(Component)]
pub struct Lock {
	timer: Timer,
}

impl Default for Lock {
	fn default() -> Self {
		Lock {
			timer: Timer::default(),
		}
	}
}

impl Lock {
	pub fn lock_for(mut self, seconds: f32) {
		self.timer = Timer::from_seconds(seconds, TimerMode::Once);
	}

	pub fn is_finished(self) -> bool {
		self.timer.finished()
	}
}

#[derive(Bundle)]
pub struct AnimationBundle {
	animation_set: AnimationSet,
	timer: AnimationTimer,
	animation: AnimationData,
	state: Player,
	lock: Lock,
}

impl Default for AnimationBundle {
	fn default() -> AnimationBundle {
		AnimationBundle {
			animation_set: AnimationSet::default(),
			timer: AnimationTimer(benimator::State::default()),
			animation: AnimationData::default(),
			state: Player::default(),
			lock: Lock::default(),
		}
	}
}

fn dir(direction: Option<Direction>) -> Direction {
	if let Some(direction) = direction {
		direction
	} else {
		Direction::Down
	}
}

impl AnimationBundle {
	pub fn new() -> AnimationBundle {
		AnimationBundle::default()
	}

	pub fn set_idling(&mut self, direction: Option<Direction>, indices: impl IntoIterator<Item=usize>) {
		self.animation_set.idling.set(dir(direction), indices);
	}

	pub fn set_moving(&mut self, direction: Option<Direction>, indices: impl IntoIterator<Item=usize>) {
		match self.animation_set.moving {
			None => {
				self.animation_set.moving = Some(AnimationDirections::from(dir(direction), indices))
			}
			Some(ref mut directions) => {
				directions.set(dir(direction), indices)
			}
		}
	}

	pub fn set_attacking(&mut self, direction: Option<Direction>, indices: impl IntoIterator<Item=usize>) {
		match self.animation_set.attacking {
			None => {
				self.animation_set.attacking = Some(AnimationDirections::from(dir(direction), indices))
			}
			Some(ref mut directions) => {
				directions.set(dir(direction), indices)
			}
		}
	}

	pub fn set_dying(&mut self, direction: Option<Direction>, indices: impl IntoIterator<Item=usize>) {
		match self.animation_set.dying {
			None => {
				self.animation_set.dying = Some(AnimationDirections::from(dir(direction), indices))
			}
			Some(ref mut directions) => {
				directions.set(dir(direction), indices)
			}
		}
	}
}
