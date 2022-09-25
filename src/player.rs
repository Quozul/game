use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::{Bullet, MainCamera, utils};

#[derive(Component)]
pub(crate) struct Movable;

#[derive(Component)]
pub(crate) struct CanShoot;

#[derive(Component)]
struct Health;

pub(crate) fn move_on_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Movable>>,
) {
    let movement_speed = 100.0;

    for mut velocity in query.iter_mut() {
        let up = keys.pressed(KeyCode::Z) || keys.pressed(KeyCode::Up);
        let down = keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down);
        let left = keys.pressed(KeyCode::Q) || keys.pressed(KeyCode::Left);
        let right = keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        velocity.linvel = move_delta * movement_speed;
    }
}

pub(crate) fn shoot_bullet(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut commands: Commands,
    mut query: Query<&Transform, With<CanShoot>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let world_pos = utils::get_world_mouse(windows, q_camera);

        for transform in query.iter_mut() {
            let angle = (world_pos.y - transform.translation.y).atan2(world_pos.x - transform.translation.x);
            let x = angle.cos();
            let y = angle.sin();

            Bullet::new(Bullet { x, y, life: 2.0 }, &mut commands, Transform::from_translation(transform.translation));
        }
    }
}