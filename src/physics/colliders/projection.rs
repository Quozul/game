use bevy::prelude::Vec2;

#[derive(Debug)]
pub struct Projection {
    min: f32,
    max: f32,
}

impl Projection {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn update(&mut self, value: f32) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
    }

    pub fn overlap(&self, other: &Self) -> bool {
        self.min <= other.max && self.max >= other.min
    }

    pub fn get_overlap(&self, other: &Self) -> f32 {
        (other.max - self.min).min(self.max - other.min)
    }
}

impl Default for Projection {
    fn default() -> Self {
        Self {
            min: f32::MAX,
            max: f32::MIN,
        }
    }
}

pub trait Project {
    fn project(&self, axis: Vec2) -> Projection;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let projection = Projection::new(1.0, 2.0);
        assert_eq!(projection.min, 1.0);
        assert_eq!(projection.max, 2.0);
    }

    #[test]
    fn test_update() {
        let mut projection = Projection::new(1.0, 2.0);
        projection.update(3.0);
        assert_eq!(projection.min, 1.0);
        assert_eq!(projection.max, 3.0);
    }

    #[test]
    fn test_overlap() {
        let projection_a = Projection::new(1.0, 4.0);
        let projection_b = Projection::new(3.0, 6.0);
        assert!(projection_a.overlap(&projection_b));
        assert!(projection_b.overlap(&projection_a));
    }

    #[test]
    fn test_not_overlap() {
        let projection_a = Projection::new(1.0, 2.0);
        let projection_b = Projection::new(3.0, 4.0);
        assert!(!projection_a.overlap(&projection_b));
        assert!(!projection_b.overlap(&projection_a));
    }

    #[test]
    fn test_get_overlap() {
        let projection_a = Projection::new(1.0, 4.0);
        let projection_b = Projection::new(3.0, 6.0);
        assert_eq!(projection_a.get_overlap(&projection_b), 1.0);
    }
}
