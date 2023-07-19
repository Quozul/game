use bevy::prelude::*;
use bevy_quinnet::shared::ClientId;

use crate::MyId;
use shared::server_entities::NetworkServerEntity;

#[derive(Event)]
pub(crate) struct UpdateYourId {
    pub(crate) id: ClientId,
}

pub(crate) fn handle_your_id_event(
    mut my_id: ResMut<MyId>,
    mut query: Query<(Entity, &NetworkServerEntity)>,
    mut event_reader: EventReader<UpdateYourId>,
) {
    for event in event_reader.iter() {
        my_id.id = event.id;
        debug!("My id is {}", event.id);

        for (entity, client_entity) in &mut query {
            if client_entity.client_id == event.id {
                my_id.entity = Some(entity);
                break;
            }
        }
    }
}
