use crate::{AppState, MyId};
use bevy::prelude::*;
use bevy_quinnet::shared::ClientId;
use shared::server_entities::NetworkServerEntity;

#[derive(Event)]
pub(crate) struct DespawnPlayerEvent {
    pub(crate) id: ClientId,
}

pub(crate) fn handle_player_despawn(
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    mut event_reader: EventReader<DespawnPlayerEvent>,
    mut my_id: ResMut<MyId>,
    query: Query<(Entity, &NetworkServerEntity)>,
) {
    for event in event_reader.iter() {
        if event.id == my_id.id {
            if let Some(entity) = my_id.entity {
                commands.entity(entity).despawn();
                my_id.entity = None;
            }

            next_state.set(AppState::Menu);
        } else {
            for (entity, network_entity) in &query {
                if event.id == network_entity.client_id {
                    commands.entity(entity).despawn();
                    break;
                }
            }
        }
    }
}