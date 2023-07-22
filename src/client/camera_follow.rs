use bevy::prelude::*;

use shared::direction::Facing;

use crate::MyId;

#[derive(Component)]
pub(crate) struct FollowSubject;

pub(crate) fn camera_follow(
    time: Res<Time>,
    my_id: ResMut<MyId>,
    mut camera_query: Query<&mut Transform, With<FollowSubject>>,
    player_query: Query<(&Facing, &Transform), Without<FollowSubject>>,
) {
    if let Some(player) = my_id.entity {
        let speed = time.delta_seconds() * 5.0;

        for mut transform in &mut camera_query {
            let player = player_query.get(player);

            if let Ok((facing, player_transform)) = player {
                let dx = player_transform.translation.x - transform.translation.x
                    + facing.angle.cos() * 10.0;
                let dy = player_transform.translation.y - transform.translation.y
                    + facing.angle.sin() * 10.0;

                transform.translation.x += dx * speed;
                transform.translation.y += dy * speed;
                transform.translation.z = 999.0;
            }
        }
    }
}
