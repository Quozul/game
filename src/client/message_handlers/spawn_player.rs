use crate::animation::AnimationBundle;
use crate::MyId;
use bevy::prelude::*;
use bevy_quinnet::shared::ClientId;
use bevy_rapier2d::prelude::{Collider, KinematicCharacterController, RigidBody};
use shared::direction::{Direction, FacingDirection, Move};
use shared::server_entities::NetworkServerEntity;

#[derive(Event)]
pub(crate) struct SpawnPlayerEvent {
    pub(crate) id: ClientId,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) you: bool,
}

pub(crate) fn handle_player_spawn(
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<SpawnPlayerEvent>,
    mut my_id: ResMut<MyId>,
) {
    for event in event_reader.iter() {
        let texture_handle = asset_server.load("characters/player.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 48.0), 6, 10, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let entity = commands
            .spawn(RigidBody::KinematicVelocityBased)
            .insert(Collider::cuboid(8.0, 8.0))
            .insert(KinematicCharacterController {
                autostep: None,
                ..default()
            })
            .insert(TransformBundle::from(Transform::from_xyz(
                event.x, event.y, 0.0,
            )))
            .insert(NetworkServerEntity {
                client_id: event.id,
            })
            .insert(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                ..default()
            })
            .insert(AnimationBundle::default())
            .insert(Move {
                direction: Direction::Idling {
                    direction: FacingDirection::Down,
                },
            })
            .id();

        if event.you {
            my_id.id = event.id;
            my_id.entity = Some(entity);
        }
    }
}
