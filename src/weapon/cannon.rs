use bevy::asset::Handle;
use bevy::math::Vec3;
use bevy::prelude::{Image, TextureAtlasLayout};

#[derive(Clone, Debug)]
pub struct Cannon {
    pub texture: Handle<Image>,
    pub offset: Vec3,
    pub recoil: f32,
    pub reload: u64,
    pub spread: f32,
    pub damage: u32,
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
}
