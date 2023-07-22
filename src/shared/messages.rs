use crate::direction::Direction;
use bevy::prelude::{Quat, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum ServerMessage {
    SpawnPlayer {
        id: u64,
        you: bool,
        x: f32,
        y: f32,
    },
    Position {
        id: u64,
        translation: Vec3,
        rotation: Quat,
    },
    Direction {
        id: u64,
        direction: Direction,
    },
    Facing {
        id: u64,
        facing: f32,
    },
    Despawn {
        id: u64,
    },
    Health {
        id: u64,
        new_health: u8,
    },
    SpawnSlime {
        id: u64,
        x: f32,
        y: f32,
    },
}

#[derive(Deserialize, Serialize)]
pub enum ClientMessage {
    Connected,
    Move { direction: Direction },
    Facing { facing: f32 },
}
