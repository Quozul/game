use crate::physics::colliders::circle_collider::CircleCollider;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct PolygonCollider {
    vertices: Vec<Vec2>,
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

    pub fn intersect_polygon(&self, other: &Self) -> Option<(Vec2, f32)> {
        self.intersect_vertices(other, Vec2::ZERO, f32::INFINITY)
            .and_then(|(normal, depth)| other.intersect_vertices(self, normal, depth))
            .map(|(normal, depth)| {
                let depth = depth / normal.length();

                let center_a = self.arithmetic_mean();
                let center_b = other.arithmetic_mean();
                (get_normalized_normal(normal, center_a, center_b), depth)
            })
    }

    pub fn intersect_circle(&self, other: &CircleCollider) -> Option<(Vec2, f32)> {
        let mut normal = Vec2::ZERO;
        let mut depth = f32::INFINITY;

        for axis in self.get_axes() {
            match self.project_circle(other, axis) {
                None => return None,
                Some(axis_depth) => {
                    if axis_depth < depth {
                        depth = axis_depth;
                        normal = axis;
                    }
                }
            }
        }

        let closest_vertex = self.get_closest_vertex(other.center);

        let axis = (closest_vertex - other.center).normalize();
        match self.project_circle(other, axis) {
            None => return None,
            Some(axis_depth) => {
                if axis_depth < depth {
                    depth = axis_depth;
                    normal = axis;
                }
            }
        }

        let polygon_center = self.arithmetic_mean();
        Some((
            get_normalized_normal(normal, other.center, polygon_center),
            depth,
        ))
    }

    fn project_circle(&self, other: &CircleCollider, axis: Vec2) -> Option<f32> {
        let (min_a, max_a) = self.project_vertices(axis);
        let (min_b, max_b) = other.project_circle(axis);

        if min_a >= max_b || min_b >= max_a {
            return None;
        }

        Some(max_b - min_a.min(max_a - min_b))
    }

    fn project_vertices(&self, axis: Vec2) -> (f32, f32) {
        self.vertices
            .iter()
            .map(|vertex| vertex.dot(axis))
            .fold((f32::INFINITY, -f32::INFINITY), |acc, projection| {
                (acc.0.min(projection), acc.1.max(projection))
            })
    }

    fn intersect_vertices(
        &self,
        other: &Self,
        mut normal: Vec2,
        mut depth: f32,
    ) -> Option<(Vec2, f32)> {
        for axis in self.get_axes() {
            // Check for collision
            let (min_a, max_a) = self.project_vertices(axis);
            let (min_b, max_b) = other.project_vertices(axis);

            if min_a >= max_b || min_b >= max_a {
                return None;
            }

            // Collision resolving
            let axis_depth = (max_b - min_a).min(max_a - min_b);

            if axis_depth < depth {
                depth = axis_depth;
                normal = axis;
            }
        }

        Some((normal, depth))
    }

    fn arithmetic_mean(&self) -> Vec2 {
        let sum = self.vertices.iter().sum::<Vec2>();
        sum / self.vertices.len() as f32
    }

    fn get_axes(&self) -> Vec<Vec2> {
        self.vertices
            .iter()
            .zip(self.vertices[1..].iter().chain(&self.vertices[0..1]))
            .map(|(va, vb)| (*vb - *va).perp())
            .collect()
    }

    fn get_closest_vertex(&self, point: Vec2) -> Vec2 {
        self.vertices
            .iter()
            .min_by(|a, b| a.distance(point).total_cmp(&b.distance(point)))
            .copied()
            .expect("Polygon must have at least one vertex")
    }

    pub fn draw_collider(&self, gizmos: &mut Gizmos) {
        self.vertices
            .iter()
            .zip(self.vertices[1..].iter().chain(&self.vertices[0..1]))
            .for_each(|(va, vb)| gizmos.line_2d(*va, *vb, Color::linear_rgb(0.0, 0.0, 1.0)));
    }

    pub fn transform(&self, transform: &Transform) -> Self {
        let adjusted_vertices = self
            .vertices
            .iter()
            .map(|vertex| transform.transform_point(vertex.extend(0.0)).xy())
            .collect::<Vec<_>>();

        PolygonCollider::polygon(adjusted_vertices)
    }
}

fn get_normalized_normal(normal: Vec2, center_a: Vec2, center_b: Vec2) -> Vec2 {
    let direction = center_b - center_a;
    if direction.dot(normal) < 0.0 {
        -normal.normalize_or_zero()
    } else {
        normal.normalize_or_zero()
    }
}
