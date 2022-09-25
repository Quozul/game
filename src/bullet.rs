use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Bullet {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) life: f32,
}

pub(crate) fn move_bullet(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Bullet, Entity)>,
) {
    let dt = time.delta_seconds();

    for (mut transform, mut bullet, entity) in query.iter_mut() {
        transform.translation.x += bullet.x * dt * 100.0;
        transform.translation.y += bullet.y * dt * 100.0;

        bullet.life -= dt;

        if bullet.life <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

impl Bullet {
    pub(crate) fn new(bullet: Bullet, commands: &mut Commands, transform: Transform) {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(5.0, 5.0)),
                ..default()
            },
            transform,
            ..default()
        })
            .insert(bullet);
    }
}