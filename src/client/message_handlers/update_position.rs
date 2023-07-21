use bevy::prelude::*;

use shared::server_entities::NetworkServerEntity;

#[derive(Event)]
pub(crate) struct UpdatePositionEvent {
    pub(crate) id: u64,
    pub(crate) translation: Vec3,
    pub(crate) rotation: Quat,
}

pub(crate) fn handle_update_position_event(
    mut query: Query<(&NetworkServerEntity, &mut Transform)>,
    mut event_reader: EventReader<UpdatePositionEvent>,
) {
    for event in event_reader.iter() {
        for (client_entity, mut transform) in &mut query {
            if client_entity.id == event.id {
                transform.translation = event.translation;
                transform.rotation = event.rotation;
                break;
            }
        }
    }
}
