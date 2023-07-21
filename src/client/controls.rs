use bevy::prelude::*;
use bevy_quinnet::client::Client;
use bevy_rapier2d::prelude::KinematicCharacterController;
use leafwing_input_manager::prelude::*;

use shared::direction::{Direction, Move};
use shared::gravity::apply_force;
use shared::messages::ClientMessage;

use crate::MyId;

fn timer_from_frame_count(frame_count: u8) -> Timer {
    Timer::from_seconds(1.0 / 10.0 * frame_count as f32, TimerMode::Once)
}

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub(crate) enum Action {
    Left,
    Right,
    Up,
    Down,
    Attacking,
}

#[derive(Component)]
pub(crate) struct AttackState {
    is_attacking: bool,
    elapsed: Timer,
}

impl Default for AttackState {
    fn default() -> Self {
        AttackState {
            is_attacking: false,
            elapsed: timer_from_frame_count(4),
        }
    }
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

pub(crate) fn attack(time: Res<Time>, mut query: Query<&mut AttackState>) {
    for mut attack_state in &mut query {
        if attack_state.is_attacking {
            attack_state.elapsed.tick(time.delta());

            if attack_state.elapsed.finished() {
                attack_state.is_attacking = false;
            }
        }
    }
}

pub(crate) fn controls(
    mut client: ResMut<Client>,
    mut query: Query<(
        &ActionState<Action>,
        &mut KinematicCharacterController,
        &mut Move,
        &mut AttackState,
    )>,
) {
    if let Some(connection) = client.get_connection_mut() {
        if let Ok((action_state, mut controller, mut move_component, mut attack_state)) =
            query.get_single_mut()
        {
            if attack_state.is_attacking {
                return;
            }

            let any_pressed = action_state.pressed(Action::Up)
                || action_state.pressed(Action::Right)
                || action_state.pressed(Action::Left)
                || action_state.pressed(Action::Down);
            let any_just_released = action_state.just_released(Action::Up)
                || action_state.just_released(Action::Right)
                || action_state.just_released(Action::Left)
                || action_state.just_released(Action::Down)
                || attack_state.is_changed();

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
                attack_state.is_attacking = true;
                attack_state.elapsed.reset();
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

                connection.try_send_message(ClientMessage::Move { direction });
            }
        }
    }
}
