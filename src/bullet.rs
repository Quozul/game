use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub(crate) struct Bullet {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) life: f32,
}

pub(crate) fn move_bullet(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&mut Bullet, Entity)>,
) {
    let dt = time.delta_seconds();

    for (mut bullet, entity) in query.iter_mut() {
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
            .insert(RigidBody::Dynamic)
            .insert(Collider::cuboid(2.5, 2.5))
            .insert(Restitution::coefficient(0.7))
            .insert(ExternalImpulse {
                impulse: Vec2::new(bullet.x, bullet.y),
                torque_impulse: 14.0,
            })
            .insert(bullet);
    }
}