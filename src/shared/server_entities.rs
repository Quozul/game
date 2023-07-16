use bevy::prelude::Resource;

#[derive(Resource)]
pub struct ServerEntityId {
    id: u64,
}

impl Default for ServerEntityId {
    fn default() -> Self {
        ServerEntityId { id: 0 }
    }
}

impl ServerEntityId {
    pub(crate) fn next_id(&mut self) -> u64 {
        self.id += 1;
        self.id
    }
}
