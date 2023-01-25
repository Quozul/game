use crate::animations::*;
use crate::player::components::{AttackTimer, FacingDirection, Player, PlayerAnimation};
use benimator::FrameRate;
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
	pub player: Player,
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
			animation_bundle: AnimationBundle::default(),
			player: Player::default(),
			external_force: ExternalImpulse {
				impulse: Vec2::ZERO,
				torque_impulse: 0.0,
			},
			health: Health(3),
		}
	}
}

pub fn update_slime_animation(
	mut query: Query<
		(
			&Player,
			&mut Animation,
			&mut AnimationData,
			&mut AnimationState,
		),
		(Changed<Player>, With<Slime>),
	>,
) {
	for (slime, mut animation, mut data, mut state) in &mut query {
		let mut flip_x = false;
		let mut once = false;

		let range = match slime.state {
			PlayerAnimation::ATTACKING => {
				state.0.reset();

				match slime.direction {
					FacingDirection::Up => 21..=23,
					FacingDirection::Down => 21..=23,
					FacingDirection::Left => {
						flip_x = true;
						21..=34
					}
					FacingDirection::Right => 21..=23,
				}
			}
			PlayerAnimation::MOVING => match slime.direction {
				FacingDirection::Up => 7..=12,
				FacingDirection::Down => 7..=12,
				FacingDirection::Left => {
					flip_x = true;
					7..=23
				}
				FacingDirection::Right => 7..=12,
			},
			PlayerAnimation::IDLING => match slime.direction {
				FacingDirection::Up => 0..=3,
				FacingDirection::Down => 0..=3,
				FacingDirection::Left => {
					flip_x = true;
					0..=3
				}
				FacingDirection::Right => 0..=3,
			},
			PlayerAnimation::DYING => {
				once = true;
				25..=30
			}
		};

		data.flip_x = flip_x;

		animation.0 = if once {
			benimator::Animation::from_indices(range, FrameRate::from_fps(10.0)).once()
		} else {
			benimator::Animation::from_indices(range, FrameRate::from_fps(10.0))
		}
	}
}
