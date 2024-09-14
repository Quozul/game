use crate::projectile::systems::{cannon_cooldown, remove_bullets, shoot_bullets};
use crate::AppState;
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (shoot_bullets, cannon_cooldown, remove_bullets).run_if(in_state(AppState::InGame)),
        );
    }
}
