use crate::enemy::components::{Enemy, Health};
use crate::physics::events::CollisionEvent;
use crate::player::components::Cooldown;
use crate::projectile::components::{Damage, Lifetime, Projectile};
use bevy::prelude::*;
use std::time::Duration;

pub fn cannon_cooldown(mut q_cooldowns: Query<&mut Cooldown>, time: Res<Time>) {
    let delta = time.delta();

    for mut cooldown in q_cooldowns.iter_mut() {
        cooldown.0 = cooldown.0.checked_sub(delta).unwrap_or(Duration::ZERO);
    }
}

// Remove projectiles

pub fn deal_projectile_damage_on_collision(
    mut commands: Commands,
    mut event: EventReader<CollisionEvent>,
    q_damages: Query<&Damage, With<Projectile>>,
    mut q_enemies: Query<&mut Health, With<Enemy>>,
) {
    for ev in event.read() {
        let damage = ev.get_from_query(&q_damages);

        if let Some(Damage(damage)) = damage {
            let enemy = ev.get_mut_from_query(&mut q_enemies);
            if let Some(mut health) = enemy {
                health.0 = health.0.saturating_sub(*damage);

                // Remove projectile on collision after hitting an enemy
                if let Some(entity) = ev.get_entity(&q_damages) {
                    commands.entity(entity).despawn()
                }
            }
        }
    }
}

// Lifetime

pub fn increment_lifetime(mut q_lifetimes: Query<&mut Lifetime>, time: Res<Time>) {
    for mut life_time in q_lifetimes.iter_mut() {
        life_time.0 += time.delta();
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
