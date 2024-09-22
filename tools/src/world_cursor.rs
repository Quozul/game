use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource, Default)]
pub struct WorldCursorPosition(pub Option<Vec2>);

pub struct WorldCursorPlugin;

impl Plugin for WorldCursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldCursorPosition::default())
            .add_systems(PreUpdate, update_cursor_position);
    }
}

fn update_cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut cursor_position: ResMut<WorldCursorPosition>,
) {
    cursor_position.0 = q_windows
        .get_single()
        .ok()
        .and_then(|window: &Window| window.cursor_position())
        .and_then(|cursor: Vec2| {
            q_camera
                .get_single()
                .ok()
                .and_then(|(camera, camera_transform)| {
                    camera.viewport_to_world(camera_transform, cursor)
                })
        })
        .map(|ray| ray.origin.truncate());
}
