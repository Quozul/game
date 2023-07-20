use bevy::prelude::{Component, Query, Res, Time, Vec2};
use bevy_rapier2d::prelude::KinematicCharacterController;
use serde::{Deserialize, Serialize};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

#[derive(Component)]
pub struct Move {
    pub direction: Direction,
}

#[derive(Deserialize, Serialize, PartialEq, Copy, Clone)]
pub enum FacingDirection {
    Up,
    Left,
    Right,
    Down,
}

impl FacingDirection {
    pub fn to_vec(&self) -> Vec2 {
        match self {
            FacingDirection::Up => Vec2::new(0.0, 1.0),
            FacingDirection::Left => Vec2::new(-1.0, 0.0),
            FacingDirection::Right => Vec2::new(1.0, 0.0),
            FacingDirection::Down => Vec2::new(0.0, -1.0),
        }
    }

    pub fn from_vec(vec: Vec2) -> FacingDirection {
        let angle = vec.y.atan2(vec.x);

        if (-FRAC_PI_4..FRAC_PI_4).contains(&angle) {
            FacingDirection::Right
        } else if (FRAC_PI_4..FRAC_PI_2).contains(&angle) {
            FacingDirection::Down
        } else if !(-FRAC_PI_2..FRAC_PI_2).contains(&angle) {
            FacingDirection::Left
        } else {
            FacingDirection::Up
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
    Idling { direction: FacingDirection },
    Attacking { direction: FacingDirection },
}

impl Direction {
    pub fn get_vec(&self) -> Option<Vec2> {
        match self {
            Direction::Up => Some(Vec2::new(0.0, 1.0)),
            Direction::Left => Some(Vec2::new(-1.0, 0.0)),
            Direction::Right => Some(Vec2::new(1.0, 0.0)),
            Direction::Down => Some(Vec2::new(0.0, -1.0)),
            _ => None,
        }
    }

    pub fn to_vec(&self) -> Vec2 {
        match self {
            Direction::Up => Vec2::new(0.0, 1.0),
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::Right => Vec2::new(1.0, 0.0),
            Direction::Down => Vec2::new(0.0, -1.0),
            _ => Vec2::ZERO,
        }
    }

    pub fn to_facing_direction(&self) -> FacingDirection {
        match self {
            Direction::Up => FacingDirection::Up,
            Direction::Left => FacingDirection::Left,
            Direction::Right => FacingDirection::Right,
            Direction::Down => FacingDirection::Down,
            Direction::Idling { direction } => *direction,
            Direction::Attacking { direction } => *direction,
        }
    }

    pub fn is_attacking(&self) -> bool {
        match self {
            Direction::Attacking { .. } => true,
            _ => false,
        }
    }
}

pub fn handle_move(
    /*time: Res<Time>, */ mut query: Query<(&mut KinematicCharacterController, &Move)>,
) {
    for (mut controller, move_component) in &mut query {
        let vel = move_component.direction.get_vec();
        controller.translation = vel;
    }
}
