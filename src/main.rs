mod utils;
mod bullet;
mod turret;
mod player;

use bevy::{prelude::*, window::PresentMode};
use bevy_rapier2d::prelude::*;
use crate::bullet::Bullet;
use crate::turret::Turret;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Game sample".to_string(),
            width: 800.,
            height: 600.,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(player::move_on_input)
        .add_system(player::shoot_bullet)
        .add_system(bullet::move_bullet)
        .add_system(turret::turret_shoot)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);

    // Floor
    commands
        .spawn()
        .insert(Collider::cuboid(500.0, 50.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    // Spawn player
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(25.0, 25.0))
        .insert(Restitution::coefficient(0.7))
        .insert(ExternalImpulse { 
            impulse: Vec2::new(0.0, 0.0), 
            torque_impulse: 0.0 
        })
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(player::Movable)
        .insert(player::CanShoot);

    // Spawn turret
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        transform: Transform::from_xyz(100.0, 0.0, 0.0),
        ..default()
    })
        .insert(Turret { shoot_delay: 100.0 });
}