use bevy::prelude::{GamepadButtonType, MouseButton, Reflect};
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::prelude::{GamepadStick, KeyboardVirtualDPad};
use leafwing_input_manager::{Actionlike, InputControlKind};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub(crate) enum PlayerAction {
    Move,
    Attack,
}

impl Actionlike for PlayerAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            PlayerAction::Move => InputControlKind::DualAxis,
            _ => InputControlKind::Button,
        }
    }
}

impl PlayerAction {
    /// Define the default bindings to the input
    pub(crate) fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad input bindings
        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT);
        input_map.insert(Self::Attack, GamepadButtonType::South);

        // Default kbm input bindings
        input_map.insert_dual_axis(Self::Move, KeyboardVirtualDPad::WASD);
        input_map.insert(PlayerAction::Attack, MouseButton::Left);

        input_map
    }
}
