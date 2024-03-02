use bevy::prelude::{Component, Resource};
use bevy_quinnet::shared::ClientId;

#[derive(Resource, Default)]
pub(crate) struct StaticServerEntity {
    id: u64,
}

impl StaticServerEntity {
    pub fn next_id(&mut self) -> u64 {
        self.id += 1;
        self.id
    }
}

#[derive(Component)]
pub struct NetworkServerEntity {
    pub id: u64,
    pub client_id: Option<ClientId>,
}
