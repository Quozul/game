use bevy::prelude::*;
use shared::direction::Move;

use shared::health::Health;

use crate::message_handlers::spawn_player::HealthDisplay;

pub(crate) fn display_health(
    query: Query<(&Health, &HealthDisplay, &Move)>,
    mut text_query: Query<&mut Text>,
) {
    for (health, display, mv) in &query {
        if let Ok(mut text) = text_query.get_mut(display.display) {
            text.sections[0].value = format!("{} HP {:?}", health.health, mv.direction);
        }
    }
}
