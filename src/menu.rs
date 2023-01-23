use crate::state_handlers::*;
use bevy::app::AppExit;
use bevy::prelude::*;

#[derive(Component)]
pub struct Counter(u32);

#[derive(Component)]
pub struct QuitGame;

pub fn quit_game(
	mut exit: EventWriter<AppExit>,
	interaction_query: Query<&Interaction, (Changed<Interaction>, With<QuitGame>)>,
) {
	for interaction in &interaction_query {
		match *interaction {
			Interaction::Clicked => {
				exit.send(AppExit);
			}
			Interaction::Hovered => {}
			Interaction::None => {}
		}
	}
}

pub fn button_cursor(
	mut windows: ResMut<Windows>,
	mut interaction_query: Query<
		(&Interaction, &mut BackgroundColor),
		(Changed<Interaction>, With<Button>),
	>,
) {
	let window = windows.primary_mut();

	for (interaction, mut color) in &mut interaction_query {
		match *interaction {
			Interaction::Clicked => {
				*color = Color::rgb(0.6, 0.6, 0.6).into();
				window.set_cursor_icon(CursorIcon::Hand);
			}
			Interaction::Hovered => {
				*color = Color::rgb(0.7, 0.7, 0.7).into();
				window.set_cursor_icon(CursorIcon::Hand);
			}
			Interaction::None => {
				*color = Color::rgb(1.0, 1.0, 1.0).into();
				window.set_cursor_icon(CursorIcon::Default);
			}
		}
	}
}

pub fn button_system(
	mut interaction_query: Query<
		(&Interaction, &Children, &mut Counter),
		(Changed<Interaction>, With<Button>),
	>,
	mut text_query: Query<&mut Text>,
) {
	for (interaction, children, mut counter) in &mut interaction_query {
		let mut text = text_query.get_mut(children[0]).unwrap();

		match *interaction {
			Interaction::Clicked => {
				text.sections[0].value = format!("{} clicks", counter.0).to_string();
				counter.0 += 1;
			}
			Interaction::Hovered => {
				text.sections[0].value = format!("{} clicks", counter.0).to_string();
			}
			Interaction::None => {
				text.sections[0].value = "Click me!".to_string();
			}
		}
	}
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands
		.spawn(Camera2dBundle::default())
		.insert(Transform::default())
		.insert(DespawnOnClose);

	commands
		.spawn(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				flex_direction: FlexDirection::Column,
				..default()
			},
			..default()
		})
		.insert(DespawnOnClose)
		.with_children(|parent| {
			parent.spawn(
				TextBundle::from_section(
					"A game",
					TextStyle {
						font: asset_server.load("fonts/LazyFox_3.ttf"),
						font_size: 30.0,
						color: Color::WHITE,
					},
				)
				.with_style(Style {
					margin: UiRect::all(Val::Px(5.0)),
					..default()
				}),
			);

			parent
				.spawn(ButtonBundle {
					style: Style {
						size: Size::AUTO,
						padding: UiRect::all(Val::Px(5.0)),
						margin: UiRect::vertical(Val::Px(5.0)),
						..default()
					},
					..default()
				})
				.insert(Counter(0))
				.with_children(|parent| {
					parent.spawn(TextBundle::from_section(
						"Click me",
						TextStyle {
							font: asset_server.load("fonts/LazyFox_4.ttf"),
							font_size: 30.0,
							color: Color::BLACK,
						},
					));
				});

			parent
				.spawn(ButtonBundle {
					style: Style {
						size: Size::AUTO,
						padding: UiRect::all(Val::Px(5.0)),
						margin: UiRect::vertical(Val::Px(5.0)),
						..default()
					},
					..default()
				})
				.insert(StateChange(AppState::InGame))
				.with_children(|parent| {
					parent.spawn(TextBundle::from_section(
						"Start game",
						TextStyle {
							font: asset_server.load("fonts/LazyFox_4.ttf"),
							font_size: 30.0,
							color: Color::BLACK,
						},
					));
				});

			parent
				.spawn(ButtonBundle {
					style: Style {
						size: Size::AUTO,
						padding: UiRect::all(Val::Px(5.0)),
						margin: UiRect::vertical(Val::Px(5.0)),
						..default()
					},
					..default()
				})
				.insert(QuitGame)
				.with_children(|parent| {
					parent.spawn(TextBundle::from_section(
						"Quit",
						TextStyle {
							font: asset_server.load("fonts/LazyFox_4.ttf"),
							font_size: 30.0,
							color: Color::BLACK,
						},
					));
				});
		});
}
