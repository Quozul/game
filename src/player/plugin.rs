use crate::player::setup::setup_player;
use crate::player::systems::{
    move_player, rotate_towards_mouse, shoot_bullets, update_velocity_display,
};
use crate::AppState;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_player)
            .add_systems(
                Update,
                (
                    move_player,
                    rotate_towards_mouse,
                    update_velocity_display,
                    shoot_bullets,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
