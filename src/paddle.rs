use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{MainCamera, utils};

#[derive(Component)]
pub(crate) struct Paddle;

pub(crate) fn cursor_events(
	windows: Res<Windows>,
	mut query: Query<&mut Transform, With<Paddle>>,
	q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
	let mouse_position = utils::get_world_mouse(windows, q_camera);

	for mut transform in query.iter_mut() {
		transform.translation.x = mouse_position.x;
	}
}

impl Paddle {
	pub(crate) fn new(commands: &mut Commands) {
		commands
			.spawn_bundle(SpriteBundle {
				sprite: Sprite {
					color: Color::rgb(0.25, 0.25, 0.75),
					custom_size: Some(Vec2::new(50.0, 10.0)),
					..default()
				},
				transform: Transform::from_xyz(0.0, -200.0, 0.0),
				..default()
			})
			.insert(RigidBody::Dynamic)
			.insert(Collider::cuboid(25.0, 5.0))
			.insert(ActiveEvents::COLLISION_EVENTS)
			.insert(LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y)
			.insert(Restitution::coefficient(1.0))
			.insert(Paddle);
	}
}