use crate::physics::colliders::center::Center;
use crate::physics::colliders::collider::{Collider, CollidesWith};
use crate::physics::collisions::collision::Collision;
use crate::physics::collisions::projection::{Project, Projection};
use bevy::math::Vec2;
use bevy::prelude::{Color, Component, Gizmos, Transform, Vec3Swizzles};

#[derive(Component, Debug)]
pub struct CircleCollider {
    radius: f32,
    center: Vec2,
}

impl CircleCollider {
    pub fn circle(radius: f32) -> Self {
        Self {
            radius,
            center: Vec2::ZERO,
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Center for CircleCollider {
    fn center(&self) -> Vec2 {
        self.center
    }
}

impl Project for CircleCollider {
    fn project(&self, axis: Vec2) -> Projection {
        assert!(
            axis.is_normalized(),
            "axis must be normalized, received {axis}"
        );
        let direction_and_radius = axis * self.radius;

        let p1 = self.center + direction_and_radius;
        let p2 = self.center - direction_and_radius;

        let f1 = p1.dot(axis);
        let f2 = p2.dot(axis);

        Projection::new(f1.min(f2), f1.max(f2))
    }
}

impl CollidesWith<CircleCollider> for CircleCollider {
    fn collides_with(&self, other: &CircleCollider) -> Option<Collision> {
        let distance = self.center.distance(other.center);
        let radii = self.radius + other.radius;

        if distance >= radii {
            return None;
        }

        let normal = (other.center - self.center).normalize();
        let depth = radii - distance;

        Some(Collision::new(normal, depth))
    }
}

impl Collider for CircleCollider {
    fn draw_collider(&self, gizmos: &mut Gizmos) {
        gizmos.circle_2d(self.center, self.radius, Color::WHITE);
    }

    fn transform(&self, transform: &Transform) -> Self {
        Self {
            radius: self.radius,
            center: transform.translation.xy(),
        }
    }
}
