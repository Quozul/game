use crate::interact::events::InteractEvent;
use crate::interact::systems::trigger_interact_event;
use bevy::prelude::*;

pub struct InteractPlugin;

impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractEvent>()
            .add_systems(Update, trigger_interact_event);
    }
}
