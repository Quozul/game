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

    /// Returns the collision with the smallest overlap
    pub fn min<'a>(&'a self, other: &'a Self) -> &'a Self {
        if self.overlap < other.overlap {
            self
        } else {
            other
        }
    }

    /// Returns the normal vector multiplied by the overlap
    /// This is the total amount the objects involved in the collision should move
    pub fn translation_vector(&self) -> Vec2 {
        self.normal * self.overlap
    }
}
