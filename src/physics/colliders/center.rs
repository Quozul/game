use bevy::prelude::Vec2;

pub trait Center {
    fn center(&self) -> Vec2;
}
