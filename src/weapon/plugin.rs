use crate::weapon::systems::{animate_weapon, update_equipped_weapon_texture};
use bevy::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (animate_weapon, update_equipped_weapon_texture));
    }
}
