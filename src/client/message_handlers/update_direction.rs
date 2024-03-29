use bevy::prelude::*;
use bevy_quinnet::shared::ClientId;

use shared::direction::{Direction, Move};
use shared::server_entities::NetworkServerEntity;

#[derive(Event)]
pub(crate) struct UpdateDirectionEvent {
    pub(crate) id: ClientId,
    pub(crate) direction: Direction,
}

pub(crate) fn handle_update_direction_event(
    mut query: Query<(&NetworkServerEntity, &mut Move)>,
    mut event_reader: EventReader<UpdateDirectionEvent>,
) {
    for event in event_reader.read() {
        for (client_entity, mut move_component) in &mut query {
            if client_entity.id == event.id {
                move_component.direction = event.direction;
                break;
            }
        }
    }
}
