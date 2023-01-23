use benimator::FrameRate;
use bevy::prelude::*;

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
pub struct AnimationState(pub benimator::State);

#[derive(Bundle)]
pub struct AnimationBundle {
	pub state: AnimationState,
	pub data: AnimationData,
	pub animation: Animation,
}

impl Default for AnimationBundle {
	fn default() -> AnimationBundle {
		AnimationBundle {
			data: AnimationData { flip_x: false },
			state: AnimationState(benimator::State::default()),
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
		&mut AnimationState,
		&mut TextureAtlasSprite,
		&Animation,
		Option<&AnimationData>,
	)>,
) {
	for (mut state, mut texture, animation, data) in query.iter_mut() {
		// Update the state
		state.update(&animation.0, time.delta());

		// Update the texture atlas
		texture.index = state.frame_index();

		if let Some(data) = data {
			texture.flip_x = data.flip_x
		}
	}
}
