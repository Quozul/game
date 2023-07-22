use bevy::prelude::*;

use shared::direction::Move;

use crate::MyId;

#[derive(Component)]
pub(crate) struct FollowSubject;

pub(crate) fn camera_follow(
    time: Res<Time>,
    my_id: ResMut<MyId>,
    mut camera_query: Query<&mut Transform, With<FollowSubject>>,
    player_query: Query<(&Move, &Transform), Without<FollowSubject>>,
) {
    if let Some(player) = my_id.entity {
        let speed = time.delta_seconds() * 5.0;

        for mut transform in &mut camera_query {
            let player = player_query.get(player);

            if let Ok((move_component, player_transform)) = player {
                let angle = move_component.facing.to_angle();
                let dx =
                    player_transform.translation.x - transform.translation.x + angle.cos() * 10.0;
                let dy =
                    player_transform.translation.y - transform.translation.y + angle.sin() * 10.0;

                transform.translation.x += dx * speed;
                transform.translation.y += dy * speed;
                transform.translation.z = 999.0;
            }
        }
    }
}
