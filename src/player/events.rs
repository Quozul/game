use benimator::FrameRate;
use bevy::prelude::{Added, Changed, Commands, Component, Entity, Input, KeyCode, Or, Query, Res, Time, Timer, With, Without};
use bevy::time::TimerMode;
use bevy_rapier2d::prelude::KinematicCharacterController;
use crate::animations::{Animation, AnimationData, AnimationState};
use crate::player::components::{FacingComponent, FacingDirection};

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Moving;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Idling;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Attacking;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Lock {
	timeout: Timer,
}

/// Remove the Lock when finished
pub fn process_locks(time: Res<Time>, mut commands: Commands, mut locks: Query<(&mut Lock, Entity)>) {
	for (mut lock, entity) in &mut locks {
		lock.timeout.tick(time.delta());
		if lock.timeout.finished() {
			commands.entity(entity).remove::<Lock>();
		}
	}
}

/// Update components when the attacking animation is done
pub fn attacking(mut commands: Commands, entities: Query<Entity, (With<Attacking>, Without<Lock>)>) {
	for entity in &entities {
		commands.entity(entity)
			.remove::<Attacking>()
			.insert(Idling);
	}
}

pub fn handle_move_event(keys: Res<Input<KeyCode>>, mut query: Query<&mut FacingComponent, (With<KinematicCharacterController>, Without<Lock>)>) {
	let just_released = keys.any_just_released(vec![KeyCode::Z, KeyCode::S, KeyCode::D, KeyCode::Q]);

	for mut facing in &mut query {
		let direction = if just_released {
			if keys.pressed(KeyCode::Z) {
				Some(FacingDirection::Up)
			} else if keys.pressed(KeyCode::S) {
				Some(FacingDirection::Down)
			} else if keys.pressed(KeyCode::D) {
				Some(FacingDirection::Right)
			} else if keys.pressed(KeyCode::Q) {
				Some(FacingDirection::Left)
			} else {
				None
			}
		} else if keys.just_pressed(KeyCode::Z) {
			Some(FacingDirection::Up)
		} else if keys.just_pressed(KeyCode::S) {
			Some(FacingDirection::Down)
		} else if keys.just_pressed(KeyCode::D) {
			Some(FacingDirection::Right)
		} else if keys.just_pressed(KeyCode::Q) {
			Some(FacingDirection::Left)
		} else {
			None
		};

		if let Some(direction) = direction {
			facing.0 = direction;
		}
	}
}

pub fn toggle_moving(keys: Res<Input<KeyCode>>, mut commands: Commands, mut query: Query<Entity, (Without<Moving>, With<KinematicCharacterController>, Without<Lock>)>) {
	let just_pressed = keys.any_just_pressed(vec![KeyCode::Z, KeyCode::S, KeyCode::D, KeyCode::Q]);

	for entity in &mut query {
		if just_pressed {
			commands.entity(entity)
				.remove::<Idling>()
				.insert(Moving);
		}
	}
}

pub fn toggle_idling(keys: Res<Input<KeyCode>>, mut commands: Commands, mut query: Query<Entity, (Without<Idling>, With<KinematicCharacterController>, Without<Lock>)>) {
	let just_released = keys.any_just_released(vec![KeyCode::Z, KeyCode::S, KeyCode::D, KeyCode::Q]);
	let pressed = keys.any_pressed(vec![KeyCode::Z, KeyCode::S, KeyCode::D, KeyCode::Q]);

	for entity in &mut query {
		if just_released && !pressed {
			commands.entity(entity)
				.remove::<Moving>()
				.insert(Idling);
		}
	}
}

pub fn attack_event(keys: Res<Input<KeyCode>>, mut commands: Commands, mut query: Query<Entity, (With<KinematicCharacterController>, Without<Lock>)>) {
	let attack_pressed = keys.just_pressed(KeyCode::Space);

	for entity in &mut query {
		if attack_pressed {
			commands.entity(entity)
				.insert(Lock {
					timeout: Timer::from_seconds(0.4, TimerMode::Once),
				})
				.remove::<Moving>()
				.remove::<Idling>()
				.insert(Attacking);
			dbg!("Now attacking");
		}
	}
}

pub fn reset_animation(mut query: Query<&mut AnimationState, Or<(Added<Moving>, Added<Idling>, Added<Attacking>)>>) {
	for mut state in &mut query {
		state.0.reset();
	}
}

type AnimQuery<'world, 'state, T> = Query<'world, 'state, (&'static FacingComponent, &'static mut Animation, &'static mut AnimationData), Or<(Added<T>, (Changed<FacingComponent>, With<T>))>>;

pub fn update_moving_animation(mut query: AnimQuery<Moving>) {
	for (facing, mut animation, mut data) in &mut query {
		let mut flip_x = false;

		let range = match facing.0 {
			FacingDirection::Up => 30..=33,
			FacingDirection::Down => 18..=23,
			FacingDirection::Left => {
				flip_x = true;
				24..=29
			}
			FacingDirection::Right => 24..=29,
		};

		data.flip_x = flip_x;

		animation.0 = benimator::Animation::from_indices(range, FrameRate::from_fps(10.0));
	}
}

pub fn update_idling_animation(mut query: AnimQuery<Idling>) {
	for (facing, mut animation, mut data) in &mut query {
		let mut flip_x = false;

		let range = match facing.0 {
			FacingDirection::Up => 12..=17,
			FacingDirection::Down => 0..=5,
			FacingDirection::Left => {
				flip_x = true;
				6..=11
			}
			FacingDirection::Right => 6..=11,
		};

		data.flip_x = flip_x;

		animation.0 = benimator::Animation::from_indices(range, FrameRate::from_fps(10.0));
	}
}

pub fn update_attacking_animation(mut query: AnimQuery<Attacking>) {
	for (facing, mut animation, mut data) in &mut query {
		let mut flip_x = false;

		let range = match facing.0 {
			FacingDirection::Up => 48..=51,
			FacingDirection::Down => 36..=39,
			FacingDirection::Left => {
				flip_x = true;
				42..=45
			}
			FacingDirection::Right => 42..=45,
		};

		data.flip_x = flip_x;

		animation.0 = benimator::Animation::from_indices(range, FrameRate::from_fps(10.0));
	}
}