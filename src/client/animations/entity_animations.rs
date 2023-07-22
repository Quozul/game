use benimator::FrameRate;
use bevy::prelude::*;

use shared::direction::{Direction, FacingDirection, Move};
use shared::player_bundle::Player;
use shared::slime_bundle::Slime;

use crate::animations::animate::{Animation, AnimationData, AnimationState};
use crate::message_handlers::spawn_player::Texture;

pub(crate) fn update_player_animation(
    mut animation_query: Query<(&mut Animation, &mut AnimationData, &mut AnimationState)>,
    query: Query<(&Texture, &Move), (Changed<Move>, Without<Slime>, With<Player>)>,
) {
    for (texture, move_component) in &query {
        if let Ok((mut animation, mut data, mut state)) = animation_query.get_mut(texture.texture) {
            let frames = match move_component.direction {
                Direction::Move { facing } => {
                    if let Some(flip_x) = facing.should_flip() {
                        data.flip_x = flip_x;
                    }

                    match facing {
                        FacingDirection::Up => 30..=33,
                        FacingDirection::Left => 24..=29,
                        FacingDirection::Right => 24..=29,
                        FacingDirection::Down => 18..=23,
                    }
                }
                Direction::Idling => {
                    if let Some(flip_x) = move_component.facing.should_flip() {
                        data.flip_x = flip_x;
                    }

                    match move_component.facing {
                        FacingDirection::Up => 12..=17,
                        FacingDirection::Down => 0..=5,
                        FacingDirection::Left => 6..=11,
                        FacingDirection::Right => 6..=11,
                    }
                }
                Direction::Attacking => {
                    if let Some(flip_x) = move_component.facing.should_flip() {
                        data.flip_x = flip_x;
                    }

                    match move_component.facing {
                        FacingDirection::Up => 48..=51,
                        FacingDirection::Left => 42..=45,
                        FacingDirection::Right => 42..=45,
                        FacingDirection::Down => 36..=39,
                    }
                }
            };

            animation.0 = benimator::Animation::from_indices(frames, FrameRate::from_fps(10.0));

            state.0.reset();
        }
    }
}

pub(crate) fn update_slime_animation(
    mut animation_query: Query<(&mut Animation, &mut AnimationData, &mut AnimationState)>,
    query: Query<(&Texture, &Move), (Changed<Move>, With<Slime>, Without<Player>)>,
) {
    for (texture, move_component) in &query {
        if let Ok((mut animation, mut data, mut state)) = animation_query.get_mut(texture.texture) {
            if let Some(flip_x) = move_component.facing.should_flip() {
                data.flip_x = flip_x;
            }

            let frames = match move_component.direction {
                Direction::Move { .. } => 7..=12,
                Direction::Idling { .. } => 0..=3,
                Direction::Attacking { .. } => 17..=20,
            };

            animation.0 = benimator::Animation::from_indices(frames, FrameRate::from_fps(10.0));

            state.0.reset();
        }
    }
}
