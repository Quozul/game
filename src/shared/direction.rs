use std::f32::consts::PI;

use bevy::prelude::{Component, Query, Res, Time, Vec2};
use bevy_rapier2d::prelude::KinematicCharacterController;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Move {
    pub direction: Direction,
    pub facing: FacingDirection,
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

    pub fn from_angle(angle: f32) -> FacingDirection {
        let normalized_angle = (angle + 2.0 * PI) % (2.0 * PI);

        if normalized_angle < PI / 4.0 || normalized_angle >= 7.0 * PI / 4.0 {
            FacingDirection::Right
        } else if normalized_angle >= PI / 4.0 && normalized_angle < 3.0 * PI / 4.0 {
            FacingDirection::Up
        } else if normalized_angle >= 3.0 * PI / 4.0 && normalized_angle < 5.0 * PI / 4.0 {
            FacingDirection::Left
        } else {
            FacingDirection::Down
        }
    }

    pub fn from_vec(vec: Vec2) -> FacingDirection {
        let angle = vec.y.atan2(vec.x);
        Self::from_angle(angle)
    }

    pub fn should_flip(&self) -> Option<bool> {
        match self {
            FacingDirection::Up => None,
            FacingDirection::Left => Some(true),
            FacingDirection::Right => Some(false),
            FacingDirection::Down => None,
        }
    }

    pub fn to_angle(&self) -> f32 {
        let vec = self.to_vec();
        vec.y.atan2(vec.x)
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
    Move { facing: FacingDirection },
    Idling,
    Attacking,
}

impl Direction {
    pub fn get_vec(&self) -> Option<Vec2> {
        match self {
            Direction::Move { facing } => Some(facing.to_vec()),
            _ => None,
        }
    }

    pub fn to_facing_direction(&self) -> FacingDirection {
        match self {
            Direction::Move { facing } => *facing,
            _ => FacingDirection::Down,
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
            0 => Direction::Move {
                facing: rand::random(),
            },
            1 => Direction::Idling,
            _ => Direction::Attacking,
        }
    }
}

const SPEED: f32 = 0.1;

pub fn handle_move(time: Res<Time>, mut query: Query<(&mut KinematicCharacterController, &Move)>) {
    for (mut controller, move_component) in &mut query {
        if let Some(vel) = move_component.direction.get_vec() {
            controller.translation = Some(vel * time.delta().as_millis() as f32 * SPEED);
        } else {
            controller.translation = None
        }
    }
}
