use crate::constants::DESPAWN_PROJECTILES;
use crate::enemy::components::{Enemy, Health};
use crate::physics::events::CollisionEvent;
use crate::player::components::Cooldown;
use crate::projectile::components::{Damage, Lifetime};
use bevy::prelude::*;
use std::time::Duration;

pub fn cannon_cooldown(mut q_cooldowns: Query<&mut Cooldown>, time: Res<Time>) {
    let delta = time.delta();

    for mut cooldown in q_cooldowns.iter_mut() {
        cooldown.0 = cooldown.0.checked_sub(delta).unwrap_or(Duration::ZERO);
    }
}

const MAX_PROJECTILE_LIFE: u64 = 1;

pub fn remove_bullets_of_old_age(
    mut commands: Commands,
    q_projectiles: Query<(&Lifetime, Entity), With<Damage>>,
) {
    for (velocity, entity) in q_projectiles.iter() {
        if velocity.0.as_secs() > MAX_PROJECTILE_LIFE {
            commands.entity(entity).despawn();
        }
    }
}

pub fn remove_bullets_on_collision(
    mut command: Commands,
    mut event: EventReader<CollisionEvent>,
    q_projectiles: Query<&Damage>,
    mut q_enemies: Query<&mut Health, With<Enemy>>,
) {
    for ev in event.read() {
        handle_collision(
            &mut command,
            &q_projectiles,
            &mut q_enemies,
            ev.first,
            ev.second,
        );
        handle_collision(
            &mut command,
            &q_projectiles,
            &mut q_enemies,
            ev.second,
            ev.first,
        );
    }
}

fn handle_collision(
    command: &mut Commands,
    q_projectiles: &Query<&Damage>,
    q_enemies: &mut Query<&mut Health, With<Enemy>>,
    projectile_entity: Entity,
    enemy_entity: Entity,
) {
    if let Ok(projectile) = q_projectiles.get(projectile_entity) {
        if DESPAWN_PROJECTILES {
            command.entity(projectile_entity).despawn();
        }

        if let Ok(mut health) = q_enemies.get_mut(enemy_entity) {
            health.0 = health.0.saturating_sub(projectile.0);
        }
    }
}

pub fn increment_lifetime(mut q_lifetimes: Query<&mut Lifetime>, time: Res<Time>) {
    for mut life_time in q_lifetimes.iter_mut() {
        life_time.0 += time.delta();
    }
}
