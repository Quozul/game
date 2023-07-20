use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_quinnet::shared::ClientId;

use shared::PlayerBundle::PlayerBundle;

use crate::animation::AnimationBundle;
use crate::controls::AttackState;
use crate::MyId;

#[derive(Event)]
pub(crate) struct SpawnPlayerEvent {
    pub(crate) id: ClientId,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) you: bool,
}

#[derive(Component)]
pub(crate) struct HealthDisplay {
    pub(crate) display: Entity,
}

pub(crate) fn handle_player_spawn(
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<SpawnPlayerEvent>,
    mut my_id: ResMut<MyId>,
) {
    for event in event_reader.iter() {
        let texture_handle = asset_server.load("characters/player.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 48.0), 6, 10, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let text_style = TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
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

        let entity = commands
            .spawn(PlayerBundle::from_spawn_event(event.id, event.x, event.y))
            .insert(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                ..default()
            })
            .insert(AnimationBundle::default())
            .insert(AttackState::default())
            .insert(HealthDisplay {
                display: health_display,
            })
            .add_child(health_display)
            .id();

        if event.you {
            my_id.id = event.id;
            my_id.entity = Some(entity);
        }
    }
}
