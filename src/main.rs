mod animations;
mod collisions;
mod game;
mod menu;
mod player;
mod slimes;
mod state_handlers;

use crate::animations::animate;
use crate::collisions::*;
use crate::game::*;
use crate::menu::*;
use crate::player::animations::*;
use crate::player::attack::{attack_enemies, dying_animation};
use crate::player::controls::{controls, movements};
use crate::slimes::*;
use crate::state_handlers::*;
use bevy::app::App;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
		.add_plugin(LdtkPlugin)
		.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
		.add_plugin(RapierDebugRenderPlugin::default())
		.insert_resource(RapierConfiguration {
			gravity: Vect::new(0.0, 0.0),
			..default()
		})
		.add_state(AppState::MainMenu)
		.add_system(change_state)
		.add_system(button_cursor)
		.add_system(quit_game)
		.add_system(animate)
		.insert_resource(LevelSelection::Index(0))
		.register_ldtk_int_cell::<WallBundle>(1)
		.register_ldtk_entity::<SlimeBundle>("Slime")
		.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_menu))
		.add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(button_system))
		.add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(despawn_entities))
		.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_game))
		.add_system_set(
			SystemSet::on_update(AppState::InGame)
				.with_system(attack_enemies)
				.with_system(dying_animation)
				.with_system(update_animation)
				.with_system(controls)
				.with_system(update_slime_animation)
				.with_system(attack_animation)
				.with_system(movements)
				.with_system(camera_follow)
				.with_system(spawn_wall_collision),
		)
		.add_system_set(SystemSet::on_exit(AppState::InGame).with_system(despawn_entities))
		.run();
}
