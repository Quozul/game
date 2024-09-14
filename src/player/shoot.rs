use crate::camera::components::MainCamera;
use crate::player::components::Player;
use crate::utils::get_mouse_world_position::{
    get_mouse_world_position, get_mouse_world_position_from_queries,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn shoot_bullets(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    q_cannons: Query<&mut Transform, With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if let Some(world_position) = get_mouse_world_position_from_queries(q_windows, q_camera) {
        if mouse_input.pressed(MouseButton::Left) {
            //
        }
    }
}
