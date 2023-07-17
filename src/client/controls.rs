use benimator::FrameRate;
use bevy::prelude::*;
use bevy_quinnet::client::Client;
use bevy_rapier2d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};
use leafwing_input_manager::prelude::*;

use shared::direction::Direction;
use shared::gravity::apply_force;
use shared::messages::ClientMessage;

use crate::animation::{Animation, AnimationData, AnimationState};
use crate::MyId;

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub(crate) enum Action {
    Left,
    Right,
    Jump,
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
                        (KeyCode::Space, Action::Jump),
                    ]),
                });
        }
    }
}

pub(crate) fn jump(
    mut client: ResMut<Client>,
    mut query: Query<(
        &ActionState<Action>,
        &mut KinematicCharacterController,
        &KinematicCharacterControllerOutput,
        &mut Animation,
        &mut AnimationData,
        &mut AnimationState,
    )>,
) {
    if let Some(connection) = client.get_connection_mut() {
        if let Ok((action_state, mut controller, output, mut animation, mut data, mut state)) =
            query.get_single_mut()
        {
            let direction = if action_state.just_pressed(Action::Jump) && output.grounded {
                animation.0 = benimator::Animation::from_indices(14..22, FrameRate::from_fps(10.0));
                Some(Direction::Jump)
            } else if action_state.pressed(Action::Right) {
                animation.0 = benimator::Animation::from_indices(8..13, FrameRate::from_fps(10.0));
                data.flip_x = false;
                Some(Direction::Right)
            } else if action_state.pressed(Action::Left) {
                animation.0 = benimator::Animation::from_indices(8..13, FrameRate::from_fps(10.0));
                data.flip_x = true;
                Some(Direction::Left)
            } else {
                animation.0 = benimator::Animation::from_indices(0..4, FrameRate::from_fps(10.0));
                None
            };

            if action_state.just_pressed(Action::Jump)
                || action_state.just_pressed(Action::Right)
                || action_state.just_pressed(Action::Left)
            {
                state.0.reset();
            }

            if let Some(direction) = direction {
                let vec = direction.to_vec();

                apply_force(&mut controller, vec);

                connection
                    .send_message(ClientMessage::Move { direction })
                    .unwrap();
            }
        }
    }
}
