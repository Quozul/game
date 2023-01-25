use benimator::FrameRate;
use bevy::prelude::*;
use crate::animations::components::AnimationSet;
use crate::player::components::Player;

// Create the animation component
// Note: you may make the animation an asset instead of a component
#[derive(Component, Deref)]
pub struct Animation(pub benimator::Animation);

#[derive(Default, Component)]
pub struct AnimationData {
	pub flip_x: bool,
}

// Create the player component
#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationTimer(pub benimator::State);

#[derive(Bundle)]
pub struct AnimationBundle {
	pub state: AnimationTimer,
	pub data: AnimationData,
	pub animation: Animation,
}

impl Default for AnimationBundle {
	fn default() -> AnimationBundle {
		AnimationBundle {
			data: AnimationData { flip_x: false },
			state: AnimationTimer(benimator::State::default()),
			animation: Animation(benimator::Animation::from_indices(
				0..=0,
				FrameRate::from_fps(10.0),
			)),
		}
	}
}

pub fn animate(
	time: Res<Time>,
	mut query: Query<(
		&mut AnimationTimer,
		&mut TextureAtlasSprite,
		&AnimationSet,
		&Player,
	)>,
) {
	for (mut state, mut texture, animation_set, player) in query.iter_mut() {
		let data = animation_set.get_animation(player.state, player.direction);

		// Update the state
		state.update(&data.animation, time.delta());

		// Update the texture atlas
		texture.index = state.frame_index();

		texture.flip_x = data.flip_x;
	}
}
