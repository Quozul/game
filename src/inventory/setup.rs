use crate::inventory::item::ItemBundle;
use crate::weapon::cannon::Cannon;
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{ColorMaterial, Commands, Ellipse, Mesh, ResMut};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let small_bullet_texture = meshes.add(Ellipse::new(1.0, 1.0));

    // Spawn an item
    commands.spawn(ItemBundle::new(
        Vec2::new(100.0, 0.0),
        Cannon {
            mesh_handle: small_bullet_texture.clone(),
            offset: Vec3::new(0.0, -25.0, 1.0),
            material_handle: materials.add(Color::linear_rgb(10.0, 10.0, 10.0)),
            recoil: 200.0,
            reload: 200,
            spread: 2.0,
        },
    ));
    commands.spawn(ItemBundle::new(
        Vec2::new(500.0, 0.0),
        Cannon {
            mesh_handle: small_bullet_texture.clone(),
            offset: Vec3::new(0.0, -25.0, 1.0),
            material_handle: materials.add(Color::linear_rgb(10.0, 10.0, 10.0)),
            recoil: 200.0,
            reload: 5,
            spread: 2.0,
        },
    ));
}
