use bevy::prelude::*;
use bevy_quinnet::client::Client;
use leafwing_input_manager::prelude::*;

use shared::direction::{Direction, FacingDirection, Move};
use shared::messages::ClientMessage;

use crate::camera_follow::FollowSubject;
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
        &Transform,
        &ActionState<Action>,
        &mut Move,
        &mut AttackState,
    )>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<FollowSubject>>,
) {
    if let Some(connection) = client.get_connection_mut() {
        if let Ok((transform, action_state, mut move_component, mut attack_state)) =
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
                Some(Direction::Move {
                    facing: FacingDirection::Up,
                })
            } else if action_state.just_pressed(Action::Right)
                || any_just_released && action_state.pressed(Action::Right)
            {
                Some(Direction::Move {
                    facing: FacingDirection::Right,
                })
            } else if action_state.just_pressed(Action::Left)
                || any_just_released && action_state.pressed(Action::Left)
            {
                Some(Direction::Move {
                    facing: FacingDirection::Left,
                })
            } else if action_state.just_pressed(Action::Down)
                || any_just_released && action_state.pressed(Action::Down)
            {
                Some(Direction::Move {
                    facing: FacingDirection::Down,
                })
            } else if action_state.just_pressed(Action::Attacking)
                || any_just_released && action_state.pressed(Action::Attacking)
            {
                attack_state.is_attacking = true;
                attack_state.elapsed.reset();
                Some(Direction::Attacking)
            } else {
                if any_just_released && !any_pressed {
                    Some(Direction::Idling)
                } else {
                    None
                }
            };

            if let Some(direction) = direction {
                move_component.direction = direction;

                connection.try_send_message(ClientMessage::Move {
                    direction: move_component.direction,
                    facing: move_component.facing,
                });
            } else if let Ok(window) = windows.get_single() {
                let (camera, camera_transform) = camera_q.single();

                if let Some(world_position) = window
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    let angle = (world_position.y - transform.translation.y)
                        .atan2(world_position.x - transform.translation.x);

                    let facing = FacingDirection::from_angle(angle);

                    if move_component.facing != facing {
                        move_component.facing = facing;

                        connection.try_send_message(ClientMessage::Move {
                            direction: move_component.direction,
                            facing: move_component.facing,
                        });
                    }
                }
            }
        }
    }
}
