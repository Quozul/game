use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub(crate) struct Brick;

impl Brick {
	pub(crate) fn new(commands: &mut Commands) {
		commands
			.spawn_bundle(SpriteBundle {
				sprite: Sprite {
					color: Color::rgb(1.0, 1.0, 1.0),
					custom_size: Some(Vec2::new(50.0, 10.0)),
					..default()
				},
				transform: Transform::from_xyz(0.0, 50.0, 0.0),
				..default()
			})
			.insert(RigidBody::Dynamic)
			.insert(Collider::cuboid(25.0, 5.0))
			.insert(ActiveEvents::COLLISION_EVENTS)
			.insert(Restitution::coefficient(1.0))
			.insert(Brick);
	}
}