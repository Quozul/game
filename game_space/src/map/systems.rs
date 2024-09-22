use crate::constants::MAP_SIZE;
use bevy::color::Color;
use bevy::math::{UVec2, Vec2};
use bevy::prelude::Gizmos;

pub fn draw_world(mut gizmos: Gizmos) {
    gizmos.grid_2d(
        Vec2::ZERO,
        0.,
        UVec2::new(10, 10),
        Vec2::splat(MAP_SIZE / 10.0),
        Color::WHITE,
    );
}
