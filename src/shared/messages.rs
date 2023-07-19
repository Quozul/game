use crate::direction::Direction;
use bevy::prelude::{Quat, Vec3};
use bevy_quinnet::shared::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum ServerMessage {
    HelloWorld {
        world: String,
    },
    Spawn {
        id: ClientId,
        you: bool,
        x: f32,
        y: f32,
    },
    Position {
        id: ClientId,
        translation: Vec3,
        rotation: Quat,
    },
    Direction {
        id: ClientId,
        direction: Direction,
    },
    Despawn {
        id: ClientId
    }
}

#[derive(Deserialize, Serialize)]
pub enum ClientMessage {
    HelloWorld { world: String },
    Connected,
    Move { direction: Direction },
}
