use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum AppState {
	MainMenu,
	InGame,
}

#[derive(Component, Deref)]
pub struct StateChange(pub AppState);

pub fn change_state(
	mut app_state: ResMut<State<AppState>>,
	interaction_query: Query<(&Interaction, &StateChange), (Changed<Interaction>, With<Button>)>,
) {
	for (interaction, state_change) in &interaction_query {
		match *interaction {
			Interaction::Clicked => {
				app_state.set(state_change.0).unwrap();
			}
			Interaction::Hovered => {}
			Interaction::None => {}
		}
	}
}

#[derive(Component)]
pub struct DespawnOnClose;

pub fn despawn_entities(mut commands: Commands, query: Query<Entity, With<DespawnOnClose>>) {
	for entity in &query {
		commands.entity(entity).despawn_recursive();
	}
}
