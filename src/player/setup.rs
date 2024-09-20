use crate::animation::components::AnimationConfig;
use crate::camera::bundle::PlayerCameraBundle;
use crate::inventory::components::Inventory;
use crate::physics::colliders::polygon_collider::PolygonCollider;
use crate::physics::components::{DragCoefficient, Force, Impulse, RigidBodyBundle};
use crate::player::components::{Cooldown, Player, VelocityDisplay};
use crate::weapon::components::EquippedWeapon;
use crate::weapon::weapon::Weapon;
use bevy::prelude::*;

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    // load the sprite sheet using the `AssetServer`
    let texture = asset_server.load("textures/Main Ship - Base - Full health.png");

    // Spawn the player
    let player_id = commands
        .spawn((
            // Sprite and animations
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                texture: texture.clone(),
                ..default()
            },
            // Physics
            PolygonCollider::square(28.0),
            VelocityDisplay(velocity_display),
            RigidBodyBundle::default()
                .with_mass(5.0)
                .with_drag_coefficient(DragCoefficient::CUBE),
            Force::default(),
            Impulse::default(),
            // Player
            Player,
            Inventory::<Weapon>::with_contents(vec![], 1),
            Cooldown::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..default()
                },
                TextureAtlas::default(),
                AnimationConfig::default(),
                EquippedWeapon,
            ));
        })
        .id();

    // Spawn the player's camera
    commands.spawn(PlayerCameraBundle::new(player_id));
}
