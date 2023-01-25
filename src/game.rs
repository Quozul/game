use std::mem::take;
use crate::player::components::{Player, Direction};
use crate::state_handlers::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::animations::components::AnimationBundle;

#[derive(Component, Deref, DerefMut, Clone, Default)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct FollowSubject(Entity);

pub fn camera_follow(
	mut camera_query: Query<(&FollowSubject, &mut Transform), Without<Player>>,
	player_query: Query<&Transform, With<Player>>,
) {
	for (subject, mut transform) in &mut camera_query {
		let player = player_query.get(subject.0);

		if let Ok(player_transform) = player {
			transform.translation = player_transform.translation
		}
	}
}

pub fn setup_game(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	// Spawn world
	commands
		.spawn(LdtkWorldBundle {
			ldtk_handle: asset_server.load("map.ldtk"),
			..default()
		})
		.insert(DespawnOnClose);

	let texture_handle = asset_server.load("characters/player.png");
	let texture_atlas = TextureAtlas::from_grid(
		texture_handle,
		Vec2::new(48.0, 48.0),
		6,
		10,
		None,
		Some(Vec2::new(0.0, 8.0)),
	);
	let texture_atlas_handle = texture_atlases.add(texture_atlas);

	let animation_bundle = &mut AnimationBundle::new();
	animation_bundle.set_idling(None, 0..=3);
	animation_bundle.set_dying(None, 25..=30);
	animation_bundle.set_attacking(Some(Direction::Down), 21..=23);

	let player = commands
		.spawn(
			SpriteSheetBundle {
				texture_atlas: texture_atlas_handle,
				..default()
			},
		)
		.insert(take(animation_bundle))
		.insert(Transform {
			translation: Vec3::new(0.0, 0.0, 5.0),
			..default()
		})
		.insert(RigidBody::Dynamic)
		.insert(Collider::cuboid(8.0, 10.0))
		.insert(DespawnOnClose)
		.insert(LockedAxes::ROTATION_LOCKED)
		.insert(Velocity {
			linvel: Vec2::new(0.0, 0.0),
			angvel: 0.0,
		})
		.id();

	commands
		.spawn(Camera2dBundle::default())
		.insert(OrthographicProjection {
			scale: 0.2,
			..default()
		})
		.insert(DespawnOnClose)
		.insert(FollowSubject(player));

	commands
		.spawn(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				justify_content: JustifyContent::FlexEnd,
				align_items: AlignItems::FlexEnd,
				flex_direction: FlexDirection::Column,
				..default()
			},
			..default()
		})
		.insert(DespawnOnClose)
		.with_children(|parent| {
			parent
				.spawn(ButtonBundle {
					style: Style {
						size: Size::AUTO,
						padding: UiRect::all(Val::Px(5.0)),
						margin: UiRect::all(Val::Px(5.0)),
						..default()
					},
					..default()
				})
				.insert(StateChange(AppState::MainMenu))
				.with_children(|parent| {
					parent.spawn(TextBundle::from_section(
						"Menu",
						TextStyle {
							font: asset_server.load("fonts/LazyFox_4.ttf"),
							font_size: 30.0,
							color: Color::BLACK,
						},
					));
				});
		});
}
