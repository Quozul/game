use bevy::prelude::*;
use bevy_quinnet::shared::ClientId;

use shared::direction::{Direction, Move};
use shared::server_entities::NetworkServerEntity;

#[derive(Event)]
pub(crate) struct UpdateDirection {
    pub(crate) id: ClientId,
    pub(crate) direction: Direction,
}

pub(crate) fn handle_update_direction_event(
    mut query: Query<(&NetworkServerEntity, &mut Move)>,
    mut event_reader: EventReader<UpdateDirection>,
) {
    for event in event_reader.iter() {
        for (client_entity, mut move_component) in &mut query {
            if client_entity.client_id == event.id {
                move_component.direction = event.direction;
                break;
            }
        }
    }
}
