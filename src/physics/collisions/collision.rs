use bevy::prelude::Vec2;

#[derive(Copy, Clone)]
pub struct Collision {
    pub normal: Vec2,
    pub overlap: f32,
}

impl Collision {
    pub fn new(normal: Vec2, overlap: f32) -> Self {
        assert!(
            normal.is_normalized(),
            "normal must be normalized, received {normal}"
        );
        Collision { normal, overlap }
    }

    pub fn min<'a>(&'a self, other: &'a Self) -> &'a Self {
        if self.overlap < other.overlap {
            self
        } else {
            other
        }
    }

    pub fn translation_vector(&self) -> Vec2 {
        self.normal * self.overlap
    }
}
