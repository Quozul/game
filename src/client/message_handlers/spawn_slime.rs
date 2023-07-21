use bevy::prelude::*;
use bevy::sprite::Anchor;

use shared::slime_bundle::SlimeBundle;

use crate::animation::AnimationBundle;
use crate::controls::AttackState;
use crate::message_handlers::spawn_player::{HealthDisplay, Texture};

#[derive(Event)]
pub(crate) struct SpawnSlimeEvent {
    pub(crate) id: u64,
    pub(crate) x: f32,
    pub(crate) y: f32,
}

pub(crate) fn handle_slime_spawn(
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<SpawnSlimeEvent>,
) {
    for event in event_reader.iter() {
        let texture_handle = asset_server.load("characters/slime.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 6, 10, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let text_style = TextStyle {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 60.0,
            color: Color::WHITE,
        };
        let text_alignment = TextAlignment::Center;

        let health_display = commands
            .spawn(Text2dBundle {
                text: Text::from_section("HP", text_style.clone()).with_alignment(text_alignment),
                text_anchor: Anchor::BottomCenter,
                text_2d_bounds: Default::default(),
                transform: Transform {
                    translation: Vec3::new(0.0, 10.0, 0.0),
                    ..default()
                },
                ..default()
            })
            .id();

        let texture = commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                ..default()
            })
            .insert(AnimationBundle::default())
            .id();

        commands
            .spawn(SlimeBundle::from_spawn_event(event.id, event.x, event.y))
            .insert(Visibility::Visible)
            .insert(ComputedVisibility::default())
            .insert(AttackState::default())
            .insert(HealthDisplay {
                display: health_display,
            })
            .insert(Texture { texture })
            .add_child(health_display)
            .add_child(texture);
    }
}
