use crate::animation::components::AnimationConfig;
use crate::inventory::components::Inventory;
use crate::weapon::components::EquippedWeapon;
use crate::weapon::weapon_data::Weapon;
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::hierarchy::Parent;
use bevy::input::ButtonInput;
use bevy::prelude::*;

pub fn update_equipped_weapon_texture(
    q_parent: Query<&Inventory<Weapon>, Changed<Inventory<Weapon>>>,
    mut q_child: Query<
        (
            &Parent,
            &mut Handle<Image>,
            &mut AnimationConfig,
            &mut TextureAtlas,
        ),
        With<EquippedWeapon>,
    >,
) {
    for (parent, mut texture, mut animation_config, mut texture_atlas) in q_child.iter_mut() {
        if let Ok(inventory) = q_parent.get(parent.get()) {
            if let Some(cannon) = inventory.get_selected_item() {
                *texture = cannon.texture_handle.clone();
                *animation_config = cannon.animation_config.clone();
                texture_atlas.layout = cannon.texture_atlas_layout.clone();
            } else {
                *texture = Default::default();
                *animation_config = Default::default();
                texture_atlas.layout = Default::default();
            }
            animation_config.stop();
            texture_atlas.index = animation_config.first_sprite_index;
        }
    }
}

pub fn animate_weapon(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut q_weapons: Query<(&mut AnimationConfig, &mut TextureAtlas), With<EquippedWeapon>>,
) {
    for (mut animation_config, mut atlas) in &mut q_weapons {
        if mouse_input.just_pressed(MouseButton::Left) {
            animation_config.start();
        } else if mouse_input.just_released(MouseButton::Left) {
            animation_config.stop();
            atlas.index = animation_config.first_sprite_index;
        }
    }
}

pub fn drop_weapon(
    mut commands: Commands,
    mut q_inventories: Query<(&Transform, &mut Inventory<Weapon>)>,
    kb_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (transform, mut inventory) in &mut q_inventories {
        if kb_input.just_pressed(KeyCode::KeyR) {
            if let Some(removed) = inventory.remove_selected_item() {
                commands.spawn(removed.config.create_item_bundle(
                    &asset_server,
                    &mut texture_atlas_layouts,
                    transform.translation,
                ));
            }
        }
    }
}
