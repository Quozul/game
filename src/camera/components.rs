use bevy::color::Color;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::{default, Bundle, Camera, Camera2dBundle, Component, Entity};

/// Used to help identify our main camera
#[derive(Component, Default)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraFollow(pub Entity);

#[derive(Bundle)]
pub struct PlayerCameraBundle {
    camera2d: Camera2dBundle,
    bloom: BloomSettings,
    main_camera: MainCamera,
    camera_follow: CameraFollow,
}

impl PlayerCameraBundle {
    pub fn new(player_id: Entity) -> Self {
        Self {
            camera2d: Camera2dBundle {
                camera: Camera {
                    hdr: true,
                    clear_color: Color::BLACK.into(),
                    ..default()
                },
                tonemapping: Tonemapping::TonyMcMapface,
                ..default()
            },
            bloom: BloomSettings::NATURAL,
            main_camera: MainCamera,
            camera_follow: CameraFollow(player_id),
        }
    }
}
