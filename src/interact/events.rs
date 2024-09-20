use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct InteractEvent {
    /// The entity that initiated the interaction
    pub initiator: Entity,
    /// The entity that has been interacted with
    pub recipient: Entity,
}
