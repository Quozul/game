use crate::camera::components::{CameraFollow, Shake};
use crate::camera::main_camera::MainCamera;
use crate::camera::post_processing::components::PostProcessSettings;
use bevy::color::Color;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

#[derive(Bundle)]
pub struct PlayerCameraBundle {
    camera_bundle: Camera2dBundle,
    bloom: BloomSettings,
    main_camera: MainCamera,
    camera_follow: CameraFollow,
    shake: Shake,
    pub post_process_settings: PostProcessSettings,
}

impl PlayerCameraBundle {
    pub fn new(player_id: Entity) -> Self {
        Self {
            camera_bundle: Camera2dBundle {
                camera: Camera {
                    hdr: true,
                    clear_color: Color::linear_rgb(0.01, 0.01, 0.01).into(),
                    ..default()
                },
                projection: OrthographicProjection {
                    far: 1000.,
                    near: -1000.,
                    // scaling_mode: ScalingMode::FixedVertical(720.0),
                    scale: 1.0,
                    ..default()
                },
                tonemapping: Tonemapping::TonyMcMapface,
                ..default()
            },
            bloom: BloomSettings::NATURAL,
            main_camera: MainCamera,
            camera_follow: CameraFollow(player_id),
            shake: Shake::default(),
            post_process_settings: PostProcessSettings {
                intensity: 0.005,
                distortion: 0.5,
            },
        }
    }
}
