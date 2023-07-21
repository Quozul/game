use bevy::prelude::{default, Bundle, Component, Transform, TransformBundle, Vec2};
use bevy_quinnet::shared::ClientId;
use bevy_rapier2d::prelude::{
    Collider, Damping, ExternalImpulse, KinematicCharacterController, LockedAxes, RigidBody,
};

use crate::direction::{Direction, FacingDirection, Move};
use crate::health::Health;
use crate::server_entities::NetworkServerEntity;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub controller: KinematicCharacterController,
    pub transform: TransformBundle,
    pub network_server_entity: NetworkServerEntity,
    pub move_component: Move,
    pub rotation_constraints: LockedAxes,
    pub external_force: ExternalImpulse,
    pub health: Health,
    pub damping: Damping,
    pub player: Player,
}

impl PlayerBundle {
    pub fn from_spawn_event(id: u64, client_id: Option<ClientId>, x: f32, y: f32) -> PlayerBundle {
        PlayerBundle {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(8.0, 8.0),
            controller: KinematicCharacterController {
                autostep: None,
                ..default()
            },
            transform: TransformBundle::from(Transform::from_xyz(x, y, 0.0)),
            network_server_entity: NetworkServerEntity { id, client_id },
            move_component: Move {
                direction: Direction::Idling {
                    direction: FacingDirection::Down,
                },
            },
            rotation_constraints: LockedAxes::ROTATION_LOCKED,
            external_force: ExternalImpulse {
                impulse: Vec2::ZERO,
                torque_impulse: 0.0,
            },
            health: Health { health: 10 },
            damping: Damping {
                linear_damping: 10.0,
                angular_damping: 10.0,
            },
            player: Player {},
        }
    }
}
