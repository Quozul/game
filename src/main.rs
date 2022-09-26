mod utils;
mod paddle;
mod ball;
mod brick;

use bevy::{prelude::*, window::PresentMode};
use bevy_rapier2d::prelude::*;
use crate::paddle::Paddle;

fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: "Game sample".to_string(),
			width: 800.,
			height: 600.,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.add_plugins(DefaultPlugins)
		.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
		.add_plugin(RapierDebugRenderPlugin::default())
		.add_startup_system(setup)

		.add_system(paddle::cursor_events)
		.add_system(ball::display_events)

		.init_resource::<RapierConfiguration>()
		.insert_resource(RapierConfiguration {
			gravity: Vect::new(0.0, 0.0),
			..Default::default()
		})
		.run();
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
	commands.spawn_bundle(Camera2dBundle::default())
		.insert(MainCamera);

	// Floor
	commands
		.spawn()
		.insert(Collider::cuboid(400.0, 50.0))
		.insert(Restitution::coefficient(1.0))
		.insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -300.0, 0.0)));

	// Ceiling
	commands
		.spawn()
		.insert(Collider::cuboid(400.0, 50.0))
		.insert(Restitution::coefficient(1.0))
		.insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 300.0, 0.0)));

	// Walls
	commands
		.spawn()
		.insert(Collider::cuboid(50.0, 300.0))
		.insert(Restitution::coefficient(1.0))
		.insert_bundle(TransformBundle::from(Transform::from_xyz(-400.0, 0.0, 0.0)));

	// Ceiling
	commands
		.spawn()
		.insert(Collider::cuboid(50.0, 300.0))
		.insert(Restitution::coefficient(1.0))
		.insert_bundle(TransformBundle::from(Transform::from_xyz(400.0, 0.0, 0.0)));

	// Spawn player
	Paddle::new(&mut commands);
	ball::Ball::new(&mut commands);
	brick::Brick::new(&mut commands);
}