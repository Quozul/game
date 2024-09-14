use crate::projectile::components::Cannon;
use bevy::asset::Handle;
use bevy::math::Vec3;
use bevy::prelude::{Bundle, ColorMaterial, Mesh};
use std::time::Duration;

#[derive(Bundle)]
pub struct CannonBundle {
    cannon: Cannon,
}

impl CannonBundle {
    pub fn new(
        mesh_handle: Handle<Mesh>,
        offset: Vec3,
        material_handle: Handle<ColorMaterial>,
    ) -> Self {
        Self {
            cannon: Cannon {
                mesh_handle,
                offset,
                material_handle,
                cooldown: Duration::ZERO,
            },
        }
    }
}
