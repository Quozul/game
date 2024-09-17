use crate::physics::colliders::center::Center;
use crate::physics::colliders::circle_collider::CircleCollider;
use crate::physics::colliders::collider::{Collider, CollidesWith};
use crate::physics::collisions::collision::Collision;
use crate::physics::collisions::projection::{Project, Projection};
use bevy::color::Color;
use bevy::prelude::{Component, Gizmos, Transform, Vec2, Vec3Swizzles};

#[derive(Component, Debug)]
pub struct PolygonCollider {
    vertices: Vec<Vec2>,
}

impl CollidesWith<PolygonCollider> for PolygonCollider {
    fn collides_with(&self, other: &PolygonCollider) -> Option<Collision> {
        let collision = self
            .intersect(other)
            .and_then(|collision| other.intersect(self).map(|other| *collision.min(&other)));

        collision
    }
}

impl CollidesWith<CircleCollider> for PolygonCollider {
    fn collides_with(&self, other: &CircleCollider) -> Option<Collision> {
        self.intersect(other).and_then(|collision| {
            self.intersect_circle(other)
                .map(|other| *collision.min(&other))
        })
    }
}

impl PolygonCollider {
    pub fn square(size: f32) -> Self {
        let half = size / 2.0;
        let vertices = vec![
            Vec2::new(-half, -half),
            Vec2::new(half, -half),
            Vec2::new(half, half),
            Vec2::new(-half, half),
        ];
        Self { vertices }
    }

    pub fn polygon(vertices: Vec<Vec2>) -> Self {
        Self { vertices }
    }

    pub fn edges(&self) -> Vec<Vec2> {
        self.vertices
            .iter()
            .zip(self.vertices[1..].iter().chain(&self.vertices[0..1]))
            .map(|(a, b)| *b - *a)
            .collect()
    }

    pub fn normals(&self) -> Vec<Vec2> {
        self.edges()
            .iter()
            .map(|edge| edge.perp().normalize_or_zero())
            .collect()
    }

    fn intersect<T>(&self, other: &T) -> Option<Collision>
    where
        T: Project + Center,
    {
        let mut collision_axis = Vec2::ZERO;
        let mut collision_overlap = f32::MAX;

        for axis in self.normals() {
            let projection_a = self.project(axis);
            let projection_b = other.project(axis);

            if !projection_a.overlap(&projection_b) {
                // then we can guarantee that the shapes do not overlap
                return None;
            }

            // find the Minimum Translation Vector
            let new_overlap = projection_a.get_overlap(&projection_b);
            if new_overlap < collision_overlap {
                collision_axis = axis;
                collision_overlap = new_overlap;
            }
        }

        let direction = self.center() - other.center();
        if direction.dot(collision_axis) < 0.0 {
            collision_axis = -collision_axis;
        }

        Some(Collision::new(collision_axis, collision_overlap))
    }

    fn intersect_circle(&self, other: &CircleCollider) -> Option<Collision> {
        let closest_point = self.find_closest_point(other.center());
        let mut axis = (closest_point - other.center()).normalize();

        let projection_a = self.project(axis);
        let projection_b = other.project(axis);

        if !projection_a.overlap(&projection_b) {
            return None;
        }

        let overlap = projection_a.get_overlap(&projection_b);

        let direction = self.center() - other.center();
        if direction.dot(axis) < 0.0 {
            axis = -axis;
        }

        Some(Collision::new(axis, overlap))
    }

    fn find_closest_point(&self, circle_center: Vec2) -> Vec2 {
        let mut closest_point = self.vertices[0];
        for vertex in &self.vertices {
            if (closest_point - circle_center).length_squared()
                > (*vertex - circle_center).length_squared()
            {
                closest_point = *vertex;
            }
        }
        closest_point
    }
}

impl Center for PolygonCollider {
    fn center(&self) -> Vec2 {
        let sum = self.vertices.iter().sum::<Vec2>();
        sum / self.vertices.len() as f32
    }
}

impl Project for PolygonCollider {
    fn project(&self, axis: Vec2) -> Projection {
        assert!(
            axis.is_normalized(),
            "axis must be normalized, received {axis}"
        );
        let mut projection = Projection::default();
        for vertex in &self.vertices {
            let v = vertex.dot(axis);
            projection.update(v);
        }
        projection
    }
}

impl Collider for PolygonCollider {
    fn draw_collider(&self, gizmos: &mut Gizmos) {
        self.vertices
            .iter()
            .zip(self.vertices[1..].iter().chain(&self.vertices[0..1]))
            .for_each(|(va, vb)| gizmos.line_2d(*va, *vb, Color::linear_rgb(0.0, 0.0, 1.0)));
    }

    fn transform(&self, transform: &Transform) -> Self {
        let adjusted_vertices = self
            .vertices
            .iter()
            .map(|vertex| transform.transform_point(vertex.extend(0.0)).xy())
            .collect::<Vec<_>>();

        PolygonCollider::polygon(adjusted_vertices)
    }
}
