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
    fn collides_with(&self, other_collider: &PolygonCollider) -> Option<Collision> {
        self.intersect(other_collider).and_then(|collision| {
            other_collider.intersect(self).map(|other| {
                let mut final_collision = *collision.min(&other);
                let normal = get_fixed_normal(final_collision.normal, other_collider, self);
                final_collision.normal = normal;
                final_collision
            })
        })
    }
}

impl CollidesWith<CircleCollider> for PolygonCollider {
    fn collides_with(&self, other_collider: &CircleCollider) -> Option<Collision> {
        self.intersect(other_collider).and_then(|collision| {
            self.intersect_circle(other_collider).map(|other| {
                let mut final_collision = *collision.min(&other);
                let normal = get_fixed_normal(final_collision.normal, other_collider, self);
                final_collision.normal = normal;
                final_collision
            })
        })
    }
}

impl PolygonCollider {
    pub fn square(size: f32) -> Self {
        Self::rectangle(size, size)
    }

    pub fn rectangle(width: f32, height: f32) -> Self {
        let vertices = vec![
            Vec2::new(-width / 2.0, -height / 2.0),
            Vec2::new(width / 2.0, -height / 2.0),
            Vec2::new(width / 2.0, height / 2.0),
            Vec2::new(-width / 2.0, height / 2.0),
        ];
        Self::polygon(vertices)
    }

    pub fn regular_polygon(circumradius: f32, sides: usize) -> Self {
        assert!(sides >= 3, "regular polygon must have at least three sides");
        let start_angle = std::f32::consts::FRAC_PI_2;
        let step = std::f32::consts::TAU / sides as f32;

        let vertices = (0..sides)
            .map(move |i| {
                let theta = start_angle + i as f32 * step;
                let (sin, cos) = theta.sin_cos();
                Vec2::new(cos, sin) * circumradius
            })
            .collect();

        Self::polygon(vertices)
    }

    pub fn polygon(vertices: Vec<Vec2>) -> Self {
        assert!(
            !vertices.is_empty(),
            "polygon collider must have at least one point"
        );
        assert!(
            vertices.iter().any(|vertex| !vertex.is_nan()),
            "polygon must contain valid vertices"
        );
        Self { vertices }
    }

    fn edges(&self) -> Vec<Vec2> {
        self.vertices
            .iter()
            .zip(self.vertices[1..].iter().chain(&self.vertices[0..1]))
            .map(|(a, b)| *b - *a)
            .collect()
    }

    fn normals(&self) -> Vec<Vec2> {
        self.edges()
            .iter()
            .map(|edge| edge.perp().normalize())
            .collect()
    }

    pub fn get_radius(&self) -> f32 {
        let center = self.center();
        let vertex = &self.vertices[0];
        let dx = center.x - vertex.x;
        let dy = center.y - vertex.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn vertices_count(&self) -> usize {
        self.vertices.len()
    }

    fn intersect<T>(&self, other: &T) -> Option<Collision>
    where
        T: Project + Center,
    {
        let mut axis = Vec2::ZERO;
        let mut overlap = f32::MAX;

        for normal in self.normals() {
            let projection_a = self.project(normal);
            let projection_b = other.project(normal);

            if !projection_a.overlap(&projection_b) {
                // then we can guarantee that the shapes do not overlap
                return None;
            }

            // find the Minimum Translation Vector
            let new_overlap = projection_a.get_overlap(&projection_b);
            if new_overlap < overlap {
                axis = normal;
                overlap = new_overlap;
            }
        }

        Some(Collision::new(axis, overlap))
    }

    fn intersect_circle(&self, other: &CircleCollider) -> Option<Collision> {
        let closest_point = self.find_closest_point(other.center());
        let axis = (closest_point - other.center()).normalize();

        let projection_a = self.project(axis);
        let projection_b = other.project(axis);

        if !projection_a.overlap(&projection_b) {
            return None;
        }

        Some(Collision::new(
            axis,
            projection_a.get_overlap(&projection_b),
        ))
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
            .for_each(|(va, vb)| gizmos.line_2d(*va, *vb, Color::WHITE));
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

fn get_fixed_normal(normal: Vec2, first: &dyn Center, second: &dyn Center) -> Vec2 {
    let direction = first.center() - second.center();
    if direction.dot(normal) < 0.0 {
        -normal
    } else {
        normal
    }
}
