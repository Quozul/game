use bevy::prelude::{default, Bundle, Component, Timer, Transform, TransformBundle, Vec2};
use bevy::time::TimerMode;
use bevy_rapier2d::prelude::{
    Collider, Damping, ExternalImpulse, KinematicCharacterController, LockedAxes, RigidBody,
};

use crate::direction::{Direction, FacingDirection, Move};
use crate::health::Health;
use crate::server_entities::NetworkServerEntity;

#[derive(Component)]
pub struct Slime {
    pub last_attack: Timer,
}

#[derive(Bundle)]
pub struct SlimeBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub controller: KinematicCharacterController,
    pub transform: TransformBundle,
    pub server_entity: NetworkServerEntity,
    pub move_component: Move,
    pub rotation_constraints: LockedAxes,
    pub external_force: ExternalImpulse,
    pub health: Health,
    pub damping: Damping,
    pub slime: Slime,
}

impl SlimeBundle {
    pub fn from_spawn_event(id: u64, x: f32, y: f32) -> SlimeBundle {
        SlimeBundle {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(8.0, 8.0),
            controller: KinematicCharacterController {
                autostep: None,
                ..default()
            },
            transform: TransformBundle::from(Transform::from_xyz(x, y, 0.0)),
            server_entity: NetworkServerEntity {
                id,
                client_id: None,
            },
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
            health: Health { health: 3 },
            damping: Damping {
                linear_damping: 10.0,
                angular_damping: 10.0,
            },
            slime: Slime {
                last_attack: Timer::from_seconds(1.0, TimerMode::Once),
            },
        }
    }
}
