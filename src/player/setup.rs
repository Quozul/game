use crate::camera::bundle::PlayerCameraBundle;
use crate::physics::colliders::polygon_collider::PolygonCollider;
use crate::physics::components::{DragCoefficient, Force, Impulse, RigidBodyBundle};
use crate::player::components::{Player, VelocityDisplay};
use crate::projectile::cannon_bundle::{CannonBundle, CreateCannon};
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
    let small_bullet_texture = meshes.add(Ellipse::new(1.0, 1.0));
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
            CannonBundle::new(vec![CreateCannon {
                mesh_handle: small_bullet_texture.clone(),
                offset: Vec3::new(0.0, -25.0, 1.0),
                material_handle: materials.add(Color::linear_rgb(10.0, 10.0, 10.0)),
                recoil: 200.0,
                reload: 200,
                spread: 2.0,
            }]),
        ))
        .id();

    // Spawn the camera
    commands.spawn(PlayerCameraBundle::new(player_id));
}
