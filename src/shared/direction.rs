use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};

use bevy::prelude::{Component, Query, Vec2};
use bevy_rapier2d::prelude::KinematicCharacterController;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Move {
    pub direction: Direction,
}

#[derive(Deserialize, Serialize, PartialEq, Copy, Clone, Debug)]
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

    pub fn should_flip(&self) -> Option<bool> {
        match self {
            FacingDirection::Up => None,
            FacingDirection::Left => Some(true),
            FacingDirection::Right => Some(false),
            FacingDirection::Down => None,
        }
    }
}

impl Distribution<FacingDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FacingDirection {
        match rng.gen_range(0..=3) {
            0 => FacingDirection::Up,
            1 => FacingDirection::Left,
            2 => FacingDirection::Right,
            _ => FacingDirection::Down,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Copy, Clone, Debug)]
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

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=5) {
            0 => Direction::Up,
            1 => Direction::Left,
            2 => Direction::Right,
            3 => Direction::Down,
            4 => Direction::Idling {
                direction: rand::random(),
            },
            _ => Direction::Attacking {
                direction: rand::random(),
            },
        }
    }
}

pub fn handle_move(mut query: Query<(&mut KinematicCharacterController, &Move)>) {
    for (mut controller, move_component) in &mut query {
        let vel = move_component.direction.get_vec();
        controller.translation = vel;
    }
}
