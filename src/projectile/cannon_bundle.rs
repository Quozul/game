use crate::projectile::components::{Cannon, CannonProperties};
use bevy::asset::Handle;
use bevy::math::Vec3;
use bevy::prelude::{Bundle, ColorMaterial, Mesh};
use std::time::Duration;

#[derive(Bundle)]
pub struct CannonBundle {
    cannon: Cannon,
}

pub struct CreateCannon {
    pub mesh_handle: Handle<Mesh>,
    pub offset: Vec3,
    pub material_handle: Handle<ColorMaterial>,
    pub recoil: f32,
    pub reload: u64,
    pub spread: f32,
}

impl CreateCannon {
    fn to_properties(&self) -> CannonProperties {
        CannonProperties {
            mesh_handle: self.mesh_handle.clone(),
            offset: self.offset,
            material_handle: self.material_handle.clone(),
            cooldown: Duration::ZERO,
            recoil: self.recoil,
            reload: self.reload,
            spread: self.spread,
        }
    }
}

impl CannonBundle {
    pub fn new(cannon_properties: Vec<CreateCannon>) -> Self {
        Self {
            cannon: Cannon {
                properties: cannon_properties
                    .iter()
                    .map(|c| c.to_properties())
                    .collect(),
            },
        }
    }
}
