use bevy::prelude::*;
use bevy_quinnet::shared::ClientId;

use shared::server_entities::NetworkServerEntity;

#[derive(Event)]
pub(crate) struct UpdatePositionEvent {
    pub(crate) id: ClientId,
    pub(crate) translation: Vec3,
    pub(crate) rotation: Quat,
}

pub(crate) fn handle_update_position_event(
    mut query: Query<(&NetworkServerEntity, &mut Transform)>,
    mut event_reader: EventReader<UpdatePositionEvent>,
) {
    for event in event_reader.iter() {
        for (client_entity, mut transform) in &mut query {
            if client_entity.client_id == event.id {
                transform.translation = event.translation;
                transform.rotation = event.rotation;
                break;
            }
        }
    }
}
