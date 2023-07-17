use bevy::prelude::*;

use crate::MyId;

#[derive(Component)]
pub struct FollowSubject;

pub fn camera_follow(
    my_id: ResMut<MyId>,
    mut camera_query: Query<&mut Transform, With<FollowSubject>>,
    player_query: Query<&Transform, Without<FollowSubject>>,
) {
    if let Some(player) = my_id.entity {
        for mut transform in &mut camera_query {
            let player = player_query.get(player);

            if let Ok(player_transform) = player {
                let dx = player_transform.translation.x - transform.translation.x;
                let dy = player_transform.translation.y - transform.translation.y;

                transform.translation.x += dx * 0.1;
                transform.translation.y += dy * 0.1;
                transform.translation.z = 999.0;
            }
        }
    }
}
