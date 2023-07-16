use crate::direction::Direction;
use bevy_quinnet::shared::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum ServerMessage {
    HelloWorld { world: String },
    Spawn { id: ClientId, x: f32, y: f32 },
    Position { id: ClientId, x: f32, y: f32 },
}

#[derive(Deserialize, Serialize)]
pub enum ClientMessage {
    HelloWorld { world: String },
    Connected,
    Move { direction: Direction },
}
