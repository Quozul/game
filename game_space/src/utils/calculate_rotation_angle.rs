use bevy::math::Vec2;
use bevy::prelude::Quat;
use std::f32::consts::FRAC_PI_2;

pub fn calculate_angle(from: Vec2, to: Vec2) -> f32 {
    (to.y - from.y).atan2(to.x - from.x)
}

pub fn calculate_rotation_angle(from: Vec2, to: Vec2) -> Quat {
    Quat::from_rotation_z(calculate_angle(from, to) - FRAC_PI_2)
}

pub fn calculate_direction_angle(from: Vec2, to: Vec2) -> Vec2 {
    Vec2::from_angle(calculate_angle(from, to))
}
