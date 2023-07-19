use benimator::FrameRate;
use bevy::prelude::*;
use bevy_quinnet::client::Client;
use bevy_rapier2d::prelude::KinematicCharacterController;
use leafwing_input_manager::prelude::*;

use shared::direction::{Direction, FacingDirection, Move};
use shared::gravity::apply_force;
use shared::messages::ClientMessage;

use crate::animation::{Animation, AnimationData, AnimationState};
use crate::MyId;

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub(crate) enum Action {
    Left,
    Right,
    Up,
    Down,
    Attacking,
}

pub(crate) fn add_controller_to_self_player(mut commands: Commands, my_id: Res<MyId>) {
    if my_id.is_changed() {
        if let Some(entity) = my_id.entity {
            commands
                .entity(entity)
                .insert(InputManagerBundle::<Action> {
                    action_state: ActionState::default(),
                    input_map: InputMap::new([
                        (KeyCode::Q, Action::Left),
                        (KeyCode::D, Action::Right),
                        (KeyCode::Z, Action::Up),
                        (KeyCode::S, Action::Down),
                        (KeyCode::Space, Action::Attacking),
                    ]),
                });
        }
    }
}

pub(crate) fn update_animation(
    mut query: Query<
        (
            &mut Animation,
            &mut AnimationData,
            &mut AnimationState,
            &Move,
        ),
        Changed<Move>,
    >,
) {
    for (mut animation, mut data, mut state, move_component) in &mut query {
        let frames = match move_component.direction {
            Direction::Up => 30..=33,
            Direction::Left => {
                data.flip_x = true;
                24..=29
            }
            Direction::Right => {
                data.flip_x = false;
                24..=29
            }
            Direction::Down => 18..=23,
            Direction::Idling { direction } => match direction {
                FacingDirection::Up => 12..=17,
                FacingDirection::Down => 0..=5,
                FacingDirection::Left => {
                    data.flip_x = true;
                    6..=11
                }
                FacingDirection::Right => {
                    data.flip_x = false;
                    6..=11
                }
            },
            Direction::Attacking { direction } => match direction {
                FacingDirection::Up => 48..=51,
                FacingDirection::Left => {
                    data.flip_x = true;
                    42..=45
                }
                FacingDirection::Right => {
                    data.flip_x = false;
                    42..=45
                }
                FacingDirection::Down => 36..=39,
            },
        };

        animation.0 = benimator::Animation::from_indices(frames, FrameRate::from_fps(10.0));

        state.0.reset();
    }
}

pub(crate) fn jump(
    mut client: ResMut<Client>,
    mut query: Query<(
        &ActionState<Action>,
        &mut KinematicCharacterController,
        &mut Move,
    )>,
) {
    if let Some(connection) = client.get_connection_mut() {
        if let Ok((action_state, mut controller, mut move_component)) = query.get_single_mut() {
            let any_pressed = action_state.pressed(Action::Up)
                || action_state.pressed(Action::Right)
                || action_state.pressed(Action::Left)
                || action_state.pressed(Action::Down);
            let any_just_released = action_state.just_released(Action::Up)
                || action_state.just_released(Action::Right)
                || action_state.just_released(Action::Left)
                || action_state.just_released(Action::Down);

            let direction = if action_state.just_pressed(Action::Up)
                || any_just_released && action_state.pressed(Action::Up)
            {
                Some(Direction::Up)
            } else if action_state.just_pressed(Action::Right)
                || any_just_released && action_state.pressed(Action::Right)
            {
                Some(Direction::Right)
            } else if action_state.just_pressed(Action::Left)
                || any_just_released && action_state.pressed(Action::Left)
            {
                Some(Direction::Left)
            } else if action_state.just_pressed(Action::Down)
                || any_just_released && action_state.pressed(Action::Down)
            {
                Some(Direction::Down)
            } else if action_state.just_pressed(Action::Attacking)
                || any_just_released && action_state.pressed(Action::Attacking)
            {
                Some(Direction::Attacking {
                    direction: move_component.direction.to_facing_direction(),
                })
            } else {
                if any_just_released && !any_pressed {
                    Some(Direction::Idling {
                        direction: move_component.direction.to_facing_direction(),
                    })
                } else {
                    None
                }
            };

            if let Some(direction) = direction {
                move_component.direction = direction;

                let vec = direction.to_vec();

                apply_force(&mut controller, vec);

                connection
                    .send_message(ClientMessage::Move { direction })
                    .unwrap();
            }
        }
    }
}
