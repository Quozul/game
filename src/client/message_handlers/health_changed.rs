use bevy::prelude::*;
use bevy_quinnet::shared::ClientId;
use shared::health::Health;
use shared::server_entities::NetworkServerEntity;

#[derive(Event)]
pub(crate) struct HealthChangedEvent {
    pub(crate) id: ClientId,
    pub(crate) new_health: u8,
}

pub(crate) fn handle_health_change(
    mut event_reader: EventReader<HealthChangedEvent>,
    mut query: Query<( &NetworkServerEntity, &mut Health)>,
) {
    for event in event_reader.iter() {
        for (network_entity, mut health) in &mut query {
            if event.id == network_entity.client_id {
                health.health = event.new_health;
                break;
            }
        }
    }
}
