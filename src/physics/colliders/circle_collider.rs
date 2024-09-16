use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct CircleCollider {
    pub radius: f32,
    pub center: Vec2,
}

impl CircleCollider {
    pub fn circle(radius: f32) -> Self {
        Self {
            radius,
            center: Vec2::ZERO,
        }
    }

    pub fn intersect_circles(&self, other: &Self) -> Option<(Vec2, f32)> {
        let distance = self.center.distance(other.center);
        let radii = self.radius + other.radius;

        if distance >= radii {
            return None;
        }

        let normal = (other.center - self.center).normalize();
        let depth = radii - distance;

        Some((normal, depth))
    }

    pub fn project_circle(&self, axis: Vec2) -> (f32, f32) {
        let direction = axis.normalize();
        let direction_and_radius = direction * self.radius;

        let p1 = self.center + direction_and_radius;
        let p2 = self.center - direction_and_radius;

        let f1 = p1.dot(axis);
        let f2 = p2.dot(axis);

        (f1.min(f2), f1.max(f2))
    }

    pub fn draw_collider(&self, gizmos: &mut Gizmos) {
        gizmos.circle_2d(self.center, self.radius, Color::WHITE);
    }

    pub fn transform(&self, transform: &Transform) -> Self {
        Self {
            radius: self.radius,
            center: transform.translation.xy(),
        }
    }
}
