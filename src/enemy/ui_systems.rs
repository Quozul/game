use crate::enemy::components::{Enemy, UiArrow};
use crate::player::components::Player;
use crate::utils::calculate_rotation_angle::calculate_angle;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

pub fn move_arrow(
    q_enemies: Query<(&Transform, &UiArrow), With<Enemy>>,
    q_player: Query<&Transform, With<Player>>,
    mut q_arrows: Query<(&mut Transform, &mut Visibility), (Without<Enemy>, Without<Player>)>,
) {
    let player_transform = q_player.single();
    let player_translation = player_transform.translation.xy();

    for (transform, arrow) in q_enemies.iter() {
        if let Ok((mut arrow_transform, mut arrow_visibility)) = q_arrows.get_mut(arrow.0) {
            let enemy_translation = transform.translation.xy();
            let distance = player_translation.distance(enemy_translation);

            if distance > 400.0 {
                let angle = calculate_angle(player_translation, enemy_translation);

                arrow_transform.rotation = Quat::from_rotation_z(angle - FRAC_PI_2);
                arrow_transform.translation = player_transform.translation
                    + Vec3::new(angle.cos() * 100.0, angle.sin() * 100.0, 0.0);
                *arrow_visibility = Visibility::Visible;
            } else {
                *arrow_visibility = Visibility::Hidden;
            }
        }
    }
}
