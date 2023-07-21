use bevy::prelude::{Component, Resource};
use bevy_quinnet::shared::ClientId;

#[derive(Resource)]
pub(crate) struct StaticServerEntity {
    id: u64,
}

impl StaticServerEntity {
    pub fn next_id(&mut self) -> u64 {
        self.id += 1;
        self.id.clone()
    }
}

impl Default for StaticServerEntity {
    fn default() -> Self {
        StaticServerEntity { id: 0 }
    }
}

#[derive(Component)]
pub struct NetworkServerEntity {
    pub id: u64,
    pub client_id: Option<ClientId>,
}
