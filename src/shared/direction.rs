use bevy::prelude::{Component, Vec2};
use serde::{Deserialize, Serialize};

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
            _ => FacingDirection::Down,
        }
    }
}
