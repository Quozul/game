use crate::message_handlers::spawn_player::HealthDisplay;
use bevy::prelude::{Query, Text, TextSection};
use shared::health::Health;

pub(crate) fn display_health(
    query: Query<(&Health, &HealthDisplay)>,
    mut text_query: Query<&mut Text>,
) {
    for (health, display) in &query {
        if let Ok(mut text) = text_query.get_mut(display.display) {
            text.sections.clear();
            text.sections.push(TextSection {
                value: format!("{} HP", health.health),
                style: Default::default(),
            });
        }
    }
}
