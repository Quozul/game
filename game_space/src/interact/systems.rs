use crate::interact::components::Interactive;
use crate::interact::events::InteractEvent;
use crate::player::components::Player;
use bevy::input::ButtonInput;
use bevy::prelude::{EventReader, EventWriter, KeyCode, Query, Res};
use tool_physics::CollisionEvent;

pub fn trigger_interact_event(
    mut collision_events: EventReader<CollisionEvent>,
    mut interact_events: EventWriter<InteractEvent>,
    q_players: Query<&Player>,
    q_interactives: Query<&Interactive>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    for ev in collision_events.read() {
        // if the interaction key is pressed
        // and a player is colliding with an interactive entity
        // then send an interaction event
        if kb_input.just_pressed(KeyCode::KeyE)
            && let Some(initiator) = ev.get_entity(&q_players)
            && let Some(recipient) = ev.get_entity(&q_interactives)
        {
            interact_events.send(InteractEvent {
                recipient,
                initiator,
            });
        }
    }
}
