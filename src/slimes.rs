use crate::player::components::{Player, Direction};
use crate::animations::components::AnimationBundle;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Slime;

#[derive(Component)]
pub struct Health(pub u8);

#[derive(Bundle)]
pub struct SlimeBundle {
	pub sprite_sheet_bundle: SpriteSheetBundle,
	pub collider: Collider,
	pub rigid_body: RigidBody,
	pub slime: Slime,
	pub rotation_constraints: LockedAxes,
	pub damping: Damping,
	pub animation_bundle: AnimationBundle,
	pub external_force: ExternalImpulse,
	pub health: Health,
}

impl LdtkEntity for SlimeBundle {
	fn bundle_entity(
		_: &EntityInstance,
		_: &LayerInstance,
		_: Option<&Handle<Image>>,
		_: Option<&TilesetDefinition>,
		asset_server: &AssetServer,
		texture_atlases: &mut Assets<TextureAtlas>,
	) -> SlimeBundle {
		let texture_handle = asset_server.load("characters/slime.png");
		let texture_atlas = TextureAtlas::from_grid(
			texture_handle,
			Vec2::new(32.0, 32.0),
			6,
			10,
			Some(Vec2::new(0.0, 0.0)),
			Some(Vec2::new(0.0, 0.0)),
		);
		let texture_atlas_handle = texture_atlases.add(texture_atlas);

		let mut animation_bundle = AnimationBundle::new();
		animation_bundle.set_idling(None, 0..=3);
		animation_bundle.set_dying(None, 25..=30);
		animation_bundle.set_attacking(Some(Direction::Down), 21..=23);

		SlimeBundle {
			sprite_sheet_bundle: SpriteSheetBundle {
				texture_atlas: texture_atlas_handle,
				..default()
			},
			collider: Collider::cuboid(8.0, 8.0),
			rigid_body: RigidBody::Dynamic,
			slime: Slime,
			rotation_constraints: LockedAxes::ROTATION_LOCKED,
			damping: Damping {
				linear_damping: 10.,
				..default()
			},
			animation_bundle,
			external_force: ExternalImpulse {
				impulse: Vec2::ZERO,
				torque_impulse: 0.0,
			},
			health: Health(3),
		}
	}
}
