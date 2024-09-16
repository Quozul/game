use crate::enemy::bundle::EnemyBundle;
use bevy::prelude::*;
use rand::Rng;

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let radius = rng.gen_range(10..100) as f32;
        let x = rng.gen_range(-3250..3250) as f32;
        let y = rng.gen_range(-3250..3250) as f32;
        commands.spawn(EnemyBundle::new(
            &mut meshes,
            &mut materials,
            radius,
            Vec3::new(x, y, 0.0),
        ));
    }
}
