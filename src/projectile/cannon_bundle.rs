use bevy::asset::Handle;
use bevy::math::Vec3;
use bevy::prelude::{ColorMaterial, Mesh};

pub struct Cannon {
    pub mesh_handle: Handle<Mesh>,
    pub offset: Vec3,
    pub material_handle: Handle<ColorMaterial>,
    pub recoil: f32,
    pub reload: u64,
    pub spread: f32,
}
