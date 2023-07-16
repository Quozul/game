use bevy::prelude::{Component, Resource};

#[derive(Component)]
pub struct ServerStaticEntityComp {
    id: u64,
}

#[derive(Resource)]
pub struct StaticServerEntity {
    id: u64,
}

impl StaticServerEntity {
    pub fn next(&mut self) -> ServerStaticEntityComp {
        let new_entity = ServerStaticEntityComp { id: self.id };

        self.id += 1;

        new_entity
    }
}

impl Default for StaticServerEntity {
    fn default() -> Self {
        StaticServerEntity { id: 0 }
    }
}

#[derive(Component)]
pub struct NetworkServerEntity {
    pub(crate) client_id: u64,
}
