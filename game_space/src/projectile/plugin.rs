use crate::projectile::systems::{
    cannon_cooldown, deal_projectile_damage_on_collision, increment_lifetime,
    remove_bullets_of_old_age,
};
use crate::AppState;
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                cannon_cooldown,
                increment_lifetime,
                remove_bullets_of_old_age,
                deal_projectile_damage_on_collision,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
