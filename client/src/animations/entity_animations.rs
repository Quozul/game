use benimator::FrameRate;
use bevy::prelude::*;

use shared::direction::{Direction, FacingDirection, Move, Rotation};
use shared::player_bundle::Player;
use shared::slime_bundle::Slime;

use crate::animations::animate::{Animation, AnimationData, AnimationState};
use crate::message_handlers::spawn_player::{FacingDirectionComponent, Texture};

pub(crate) fn flip_animation(
    mut animation_query: Query<&mut AnimationData>,
    query: Query<
        (&Texture, &Move, &FacingDirectionComponent),
        Or<(Changed<Move>, Changed<FacingDirectionComponent>)>,
    >,
) {
    for (texture, move_component, facing) in &query {
        if let Ok(mut data) = animation_query.get_mut(texture.texture) {
            let should_flip = match move_component.direction {
                Direction::Move { facing: angle } => {
                    FacingDirection::from_angle(angle.y.atan2(angle.x)).should_flip()
                }
                _ => facing.direction.should_flip(),
            };

            if let Some(should_flip) = should_flip && should_flip != data.flip_x {
                data.flip_x = should_flip;
            }
        }
    }
}

pub(crate) fn update_facing_direction(
    mut query: Query<(&mut FacingDirectionComponent, &Rotation), Changed<Rotation>>,
) {
    for (mut facing_direction, facing) in &mut query {
        let new_direction = FacingDirection::from_angle(facing.angle);

        if new_direction != facing_direction.direction {
            facing_direction.direction = new_direction;
        }
    }
}

pub(crate) fn update_player_animation(
    mut animation_query: Query<(&mut Animation, &mut AnimationState)>,
    query: Query<
        (&Texture, &Move, &FacingDirectionComponent),
        (
            Or<(Changed<Move>, Changed<FacingDirectionComponent>)>,
            Without<Slime>,
            With<Player>,
        ),
    >,
) {
    for (texture, move_component, facing) in &query {
        if let Ok((mut animation, mut state)) = animation_query.get_mut(texture.texture) {
            let frames = match move_component.direction {
                Direction::Move { facing: angle } => {
                    let facing = FacingDirection::from_angle(angle.y.atan2(angle.x));

                    match facing {
                        FacingDirection::Up => 30..=33,
                        FacingDirection::Left => 24..=29,
                        FacingDirection::Right => 24..=29,
                        FacingDirection::Down => 18..=23,
                    }
                }
                Direction::Idling => match facing.direction {
                    FacingDirection::Up => 12..=17,
                    FacingDirection::Down => 0..=5,
                    FacingDirection::Left => 6..=11,
                    FacingDirection::Right => 6..=11,
                },
                Direction::Attacking => match facing.direction {
                    FacingDirection::Up => 48..=51,
                    FacingDirection::Left => 42..=45,
                    FacingDirection::Right => 42..=45,
                    FacingDirection::Down => 36..=39,
                },
                Direction::Dying => 54..=56,
            };

            animation.0 = benimator::Animation::from_indices(frames, FrameRate::from_fps(10.0));

            state.0.reset();
        }
    }
}

pub(crate) fn update_slime_animation(
    mut animation_query: Query<(&mut Animation, &mut AnimationState)>,
    query: Query<(&Texture, &Move), (Changed<Move>, With<Slime>, Without<Player>)>,
) {
    for (texture, move_component) in &query {
        if let Ok((mut animation, mut state)) = animation_query.get_mut(texture.texture) {
            let frames = match move_component.direction {
                Direction::Move { .. } => 7..=12,
                Direction::Idling => 0..=3,
                Direction::Attacking => 17..=20,
                Direction::Dying => 24..=30,
            };

            animation.0 = benimator::Animation::from_indices(frames, FrameRate::from_fps(10.0));

            state.0.reset();
        }
    }
}
