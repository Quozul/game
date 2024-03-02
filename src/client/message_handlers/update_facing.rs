use bevy::prelude::*;
use bevy_quinnet::shared::ClientId;
use shared::direction::Rotation;

use shared::server_entities::NetworkServerEntity;

#[derive(Event)]
pub(crate) struct UpdateFacingEvent {
    pub(crate) id: ClientId,
    pub(crate) facing: f32,
}

pub(crate) fn handle_update_facing_event(
    mut query: Query<(&NetworkServerEntity, &mut Rotation)>,
    mut event_reader: EventReader<UpdateFacingEvent>,
) {
    for event in event_reader.read() {
        for (client_entity, mut facing) in &mut query {
            if client_entity.id == event.id {
                facing.angle = event.facing;
                break;
            }
        }
    }
}
