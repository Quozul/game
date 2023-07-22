use crate::direction::Direction;
use bevy::prelude::*;
use bevy_quinnet::shared::ClientId;

#[derive(Event)]
pub(crate) struct ClientConnectedEvent {
    pub(crate) client_id: ClientId,
}

#[derive(Event)]
pub(crate) struct ClientMoveEvent {
    pub(crate) direction: Direction,
    pub(crate) client_id: u64,
}

#[derive(Event)]
pub(crate) struct ClientFacingEvent {
    pub(crate) facing: f32,
    pub(crate) client_id: u64,
}
