use bevy::prelude::*;
use bevy_quinnet::client::Client;
use leafwing_input_manager::prelude::*;

use shared::direction::{Direction, Move, Rotation};
use shared::health::timer_from_frame_count;
use shared::messages::ClientMessage;

use crate::camera_follow::FollowSubject;
use crate::MyId;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub(crate) enum Action {
    Move,
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
            let mut input_map = InputMap::default();

            input_map.insert(Action::Attacking, MouseButton::Left);
            input_map.insert(Action::Attacking, KeyCode::Space);
            input_map.insert(Action::Attacking, GamepadButtonType::South);

            input_map.insert(Action::Move, VirtualDPad::arrow_keys());
            input_map.insert(Action::Move, DualAxis::left_stick());
            input_map.insert(
                Action::Move,
                VirtualDPad {
                    up: KeyCode::Z.into(),
                    down: KeyCode::S.into(),
                    left: KeyCode::Q.into(),
                    right: KeyCode::D.into(),
                },
            );

            commands.entity(entity).insert(InputManagerBundle {
                input_map,
                ..Default::default()
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
    my_id: Res<MyId>,
    mut client: ResMut<Client>,
    mut query: Query<(&ActionState<Action>, &mut Move, &mut AttackState)>,
) {
    if let Some(entity) = my_id.entity {
        if let Ok((action_state, mut move_component, mut attack_state)) = query.get_mut(entity) {
            if attack_state.is_attacking {
                return;
            }

            let direction = if action_state.pressed(&Action::Attacking) {
                attack_state.is_attacking = true;
                attack_state.elapsed.reset();
                Direction::Attacking
            } else if action_state.pressed(&Action::Move) {
                let axis_pair = action_state.axis_pair(&Action::Move).unwrap();

                Direction::Move {
                    facing: Vec2::new(axis_pair.x(), axis_pair.y()).normalize(),
                }
            } else {
                Direction::Idling
            };

            if direction != move_component.direction {
                move_component.direction = direction;

                if let Some(connection) = client.get_connection_mut() {
                    connection.try_send_message(ClientMessage::Move {
                        direction: move_component.direction,
                    });
                }
            }
        }
    }
}

pub(crate) fn mouse_controls(
    my_id: Res<MyId>,
    mut client: ResMut<Client>,
    mut query: Query<(&Transform, &mut Rotation, &Move)>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<FollowSubject>>,
) {
    if let Some(entity) = my_id.entity {
        if let Ok((transform, mut facing, move_component)) = query.get_mut(entity) {
            if let Ok(window) = windows.get_single() {
                let (camera, camera_transform) = camera_q.single();

                let angle = if let Some(world_position) = window
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    Some(
                        (world_position.y - transform.translation.y)
                            .atan2(world_position.x - transform.translation.x),
                    )
                } else if let Some(direction) = move_component.direction.get_facing_direction() {
                    Some(direction.to_angle())
                } else {
                    None
                };

                if let Some(angle) = angle
                    && (facing.angle - angle).abs() > 0.1
                {
                    facing.angle = angle;

                    if let Some(connection) = client.get_connection_mut() {
                        connection.try_send_message(ClientMessage::Facing {
                            facing: facing.angle,
                        });
                    }
                }
            }
        }
    }
}
