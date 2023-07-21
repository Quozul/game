use bevy::prelude::*;

use shared::health::Health;

use crate::message_handlers::spawn_player::HealthDisplay;

pub(crate) fn display_health(
    query: Query<(&Health, &HealthDisplay)>,
    mut text_query: Query<&mut Text>,
) {
    for (health, display) in &query {
        if let Ok(mut text) = text_query.get_mut(display.display) {
            text.sections[0].value = format!("{} HP", health.health);
        }
    }
}
