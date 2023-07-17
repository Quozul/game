use bevy::prelude::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq)]
pub enum Direction {
    Jump,
    Left,
    Right,
}

impl Direction {
    pub fn to_vec(&self) -> Vec2 {
        match self {
            Direction::Jump => Vec2::new(0.0, 50.0),
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::Right => Vec2::new(1.0, 0.0),
        }
    }
}
