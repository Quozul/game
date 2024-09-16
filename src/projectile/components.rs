use bevy::asset::Handle;
use bevy::math::Vec3;
use bevy::prelude::{ColorMaterial, Component, Mesh};
use std::time::Duration;

#[derive(Component)]
pub struct Damage(pub u32);

pub struct CannonProperties {
    pub mesh_handle: Handle<Mesh>,
    pub offset: Vec3,
    pub material_handle: Handle<ColorMaterial>,
    pub cooldown: Duration,
    pub recoil: f32,
    pub reload: u64,
    pub spread: f32,
}

#[derive(Component)]
pub struct Cannon {
    pub properties: Vec<CannonProperties>,
}

#[derive(Component, Default)]
pub struct Lifetime(pub Duration);
