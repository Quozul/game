use crate::projectile::systems::{
    cannon_cooldown, increment_lifetime, remove_bullets_of_old_age, remove_bullets_on_collision,
    shoot_bullets,
};
use crate::AppState;
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                shoot_bullets,
                cannon_cooldown,
                increment_lifetime,
                remove_bullets_of_old_age,
                remove_bullets_on_collision,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
