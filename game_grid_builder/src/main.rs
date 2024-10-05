use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use tool_physics::polygon_collider::PolygonCollider;
use tool_physics::{PhysicsPlugin, RigidBodyBundle};
use tools::{WorldCursorPlugin, WorldCursorPosition};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugin {
                newton_gravity: 0.0,
                draw_debug_colliders: true,
                ..default()
            },
            WorldCursorPlugin,
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.0, 1.0, 0.0),
                        font: default(),
                    },
                },
            },
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (update, move_camera))
        .run();
}

#[derive(Component)]
struct MainCamera;

const SIZE: i32 = 16;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle { ..default() }, MainCamera));
    for x in 0..SIZE {
        for y in 0..SIZE {
            commands.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    x as f32 * 32.0 + 16.0,
                    y as f32 * 32.0 + 16.0,
                    0.0,
                )),
                PolygonCollider::square(32.0),
                RigidBodyBundle::default(),
            ));
        }
    }
}

fn update(
    mut gizmos: Gizmos,
    mut commands: Commands,
    world_cursor_position: Res<WorldCursorPosition>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    if let Some(translation) = world_cursor_position.0 {
        let translation = (translation / 32.0).floor() * 32.0 + 16.0;

        gizmos.rect_2d(
            translation,
            0.0,
            Vec2::splat(32.0),
            Color::linear_rgba(1.0, 1.0, 1.0, 0.5),
        );

        if mouse_input.just_pressed(MouseButton::Left) {
            commands.spawn((
                TransformBundle::from_transform(Transform::from_translation(
                    translation.extend(0.),
                )),
                PolygonCollider::square(32.0),
                RigidBodyBundle::default(),
            ));
        }
    }
}

fn move_camera(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut q_cameras: Query<&mut Transform, With<MainCamera>>,
) {
    if mouse_input.pressed(MouseButton::Right) {
        for mut transform in &mut q_cameras {
            for ev in evr_motion.read() {
                transform.translation += Vec3::new(-ev.delta.x, ev.delta.y, 0.0);
            }
        }
    }
}
