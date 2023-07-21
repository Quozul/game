use bevy::prelude::*;
use bevy::sprite::Anchor;

use shared::player_bundle::PlayerBundle;

use crate::animations::animate::AnimationBundle;
use crate::controls::AttackState;
use crate::menu::AssetsLoading;
use crate::MyId;

#[derive(Event)]
pub(crate) struct SpawnPlayerEvent {
    pub(crate) id: u64,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) you: bool,
}

#[derive(Component)]
pub(crate) struct HealthDisplay {
    pub(crate) display: Entity,
}

#[derive(Component)]
pub(crate) struct Texture {
    pub(crate) texture: Entity,
}

pub(crate) fn handle_player_spawn(
    assets: ResMut<AssetsLoading>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
    mut event_reader: EventReader<SpawnPlayerEvent>,
    mut my_id: ResMut<MyId>,
) {
    for event in event_reader.iter() {
        let texture_atlas = TextureAtlas::from_grid(
            assets.player.clone().unwrap(),
            Vec2::new(48.0, 48.0),
            6,
            10,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let text_style = TextStyle {
            font: assets.font.clone().unwrap(),
            font_size: 10.0,
            color: Color::WHITE,
        };
        let text_alignment = TextAlignment::Center;

        let health_display = commands
            .spawn(Text2dBundle {
                text: Text::from_section("HP", text_style.clone()).with_alignment(text_alignment),
                text_anchor: Anchor::BottomCenter,
                text_2d_bounds: Default::default(),
                transform: Transform {
                    translation: Vec3::new(0.0, 16.0, 0.0),
                    ..default()
                },
                ..default()
            })
            .id();

        let texture = commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_xyz(0.0, 16.0, 0.0),
                ..default()
            })
            .insert(AnimationBundle::default())
            .id();

        let entity = commands
            .spawn(PlayerBundle::from_spawn_event(
                event.id, None, event.x, event.y,
            ))
            .insert(Visibility::Visible)
            .insert(ComputedVisibility::default())
            .insert(AttackState::default())
            .insert(HealthDisplay {
                display: health_display,
            })
            .insert(Texture { texture })
            .add_child(health_display)
            .add_child(texture)
            .id();

        if event.you {
            my_id.id = event.id;
            my_id.entity = Some(entity);
        }
    }
}
