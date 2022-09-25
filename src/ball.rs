use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub(crate) struct Ball;

pub(crate) fn display_events(
	mut collision_events: EventReader<CollisionEvent>,
) {
	for collision_event in collision_events.iter() {
		println!("Received collision event: {:?}", collision_event);
	}
}

impl Ball {
	pub(crate) fn new(commands: &mut Commands) {
		commands
			.spawn_bundle(SpriteBundle {
				sprite: Sprite {
					color: Color::rgb(1.0, 1.0, 1.0),
					custom_size: Some(Vec2::new(5.0, 5.0)),
					..default()
				},
				transform: Transform::from_xyz(0.0, 0.0, 0.0),
				..default()
			})
			.insert(RigidBody::Dynamic)
			.insert(Collider::ball(2.5))
			.insert(ActiveEvents::COLLISION_EVENTS)
			.insert(Restitution::coefficient(1.0))
			.insert(ExternalImpulse {
				impulse: Vec2::new(0.0, -1.0),
				torque_impulse: 0.0,
			})
			.insert(Ball);
	}
}