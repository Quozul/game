use crate::constants::{MAP_SIZE, WALL_THICKNESS};
use crate::physics::colliders::polygon_collider::PolygonCollider;
use crate::physics::movements_components::RigidBodyType;
use bevy::prelude::*;

pub fn setup_map(mut commands: Commands) {
    // Spawn walls
    commands.spawn((
        Transform::from_xyz(-(MAP_SIZE + WALL_THICKNESS) / 2.0, 0.0, 0.0),
        PolygonCollider::rectangle(WALL_THICKNESS, MAP_SIZE + WALL_THICKNESS * 2.0),
        RigidBodyType::Static,
    ));
    commands.spawn((
        Transform::from_xyz((MAP_SIZE + WALL_THICKNESS) / 2.0, 0.0, 0.0),
        PolygonCollider::rectangle(WALL_THICKNESS, MAP_SIZE + WALL_THICKNESS * 2.0),
        RigidBodyType::Static,
    ));
    commands.spawn((
        Transform::from_xyz(0.0, -(MAP_SIZE + WALL_THICKNESS) / 2.0, 0.0),
        PolygonCollider::rectangle(MAP_SIZE, WALL_THICKNESS),
        RigidBodyType::Static,
    ));
    commands.spawn((
        Transform::from_xyz(0.0, (MAP_SIZE + WALL_THICKNESS) / 2.0, 0.0),
        PolygonCollider::rectangle(MAP_SIZE, WALL_THICKNESS),
        RigidBodyType::Static,
    ));
}

pub fn setup_tutorial(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_section(
            "move: WASD\nshoot: left mouse button\nzoom: mouse wheel",
            TextStyle {
                font_size: 16.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
    );
}
