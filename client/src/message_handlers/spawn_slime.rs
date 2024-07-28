use bevy::prelude::*;
use bevy::sprite::Anchor;

use shared::slime_bundle::SlimeBundle;

use crate::animations::animate::AnimationBundle;
use crate::controls::AttackState;
use crate::menu::AssetsLoading;
use crate::message_handlers::spawn_player::{HealthDisplay, Texture};

#[derive(Event)]
pub(crate) struct SpawnSlimeEvent {
    pub(crate) id: u64,
    pub(crate) x: f32,
    pub(crate) y: f32,
}

pub(crate) fn handle_slime_spawn(
    assets: ResMut<AssetsLoading>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
    mut event_reader: EventReader<SpawnSlimeEvent>,
) {
    for event in event_reader.read() {
        let texture_handle = assets.slime.clone().unwrap();
        let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 6, 10, None, None);
        let layout_handle = texture_atlases.add(layout);

        let text_style = TextStyle {
            font: assets.font.clone().unwrap(),
            font_size: 10.0,
            color: Color::WHITE,
        };
        let text_alignment = JustifyText::Center;

        let health_display = commands
            .spawn(Text2dBundle {
                text: Text::from_section("HP", text_style.clone()).with_justify(text_alignment),
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
                sprite: Sprite::default(),
                atlas: TextureAtlas {
                    layout: layout_handle,
                    index: 0,
                },
                texture: texture_handle,
                ..default()
            })
            .insert(AnimationBundle::default())
            .id();

        commands
            .spawn(SlimeBundle::from_spawn_event(event.id, event.x, event.y))
            .insert(Visibility::Visible)
            .insert(InheritedVisibility::default())
            .insert(AttackState::default())
            .insert(HealthDisplay {
                display: health_display,
            })
            .insert(Texture { texture })
            .add_child(health_display)
            .add_child(texture);
    }
}
