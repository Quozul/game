use crate::enemy::health_system::despawn_dead;
use crate::enemy::movement::move_enemy;
use crate::enemy::setup::spawn_enemy;
use crate::AppState;
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_enemy)
            .add_systems(
                Update,
                (move_enemy, despawn_dead).run_if(in_state(AppState::InGame)),
            );
    }
}
