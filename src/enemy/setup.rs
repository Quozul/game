use crate::constants::{ASTEROID_COUNT, HALF_MAP};
use crate::enemy::bundle::EnemyBundle;
use bevy::prelude::*;
use rand::Rng;

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for _ in 0..ASTEROID_COUNT {
        let radius = rng.gen_range(50..100) as f32;
        let x = rng.gen_range(-HALF_MAP..HALF_MAP);
        let y = rng.gen_range(-HALF_MAP..HALF_MAP);
        let sides = rng.gen_range(3..=9);
        commands.spawn(EnemyBundle::new(
            &mut meshes,
            &mut materials,
            radius,
            sides,
            Vec3::new(x, y, 0.0),
            Vec2::ZERO,
        ));
    }
}
