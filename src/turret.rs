use bevy::prelude::*;
use crate::Bullet;

#[derive(Component)]
pub(crate) struct Turret {
    pub(crate) shoot_delay: f32,
}

pub(crate) fn turret_shoot(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Turret)>,
) {
    let dt = time.delta_seconds();

    for (mut transform, mut turret) in query.iter_mut() {
        turret.shoot_delay -= dt;

        // Shoot every seconds
        if turret.shoot_delay <= 0.0 {
            let angle: f32 = 0.0;
            let x = angle.cos();
            let y = angle.sin();

            Bullet::new(Bullet { x, y, life: 2.0 }, &mut commands, Transform::from_translation(transform.translation));

            turret.shoot_delay = 1.0;
        }
    }
}