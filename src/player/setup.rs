use crate::camera::bundle::PlayerCameraBundle;
use crate::inventory::components::Inventory;
use crate::physics::colliders::polygon_collider::PolygonCollider;
use crate::physics::components::{DragCoefficient, Force, Impulse, RigidBodyBundle};
use crate::player::components::{Cooldown, Player, VelocityDisplay};
use crate::weapon::cannon::Cannon;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn speed text
    let text_style = TextStyle {
        font_size: 16.0,
        ..default()
    };
    let velocity_display = commands
        .spawn((TextBundle::from_sections(vec![
            TextSection::new("speed: ", text_style.clone()),
            TextSection::new("0", text_style),
        ])
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),))
        .id();

    // Spawn the player
    let rect_mesh = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 50.0)));
    let player_id = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: rect_mesh,
                material: materials.add(Color::linear_rgb(0.0, 0.0, 1.0)),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            },
            PolygonCollider::square(50.0),
            Player,
            VelocityDisplay(velocity_display),
            RigidBodyBundle::default()
                .with_mass(5.0)
                .with_drag_coefficient(DragCoefficient::CUBE),
            Force::default(),
            Impulse::default(),
            Inventory::<Cannon>::with_contents(vec![]),
            Cooldown::default(),
        ))
        .id();

    // Spawn the camera
    commands.spawn(PlayerCameraBundle::new(player_id));
}
