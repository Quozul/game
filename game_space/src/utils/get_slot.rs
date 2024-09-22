use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Res};

const KEY_MAP: [(KeyCode, KeyCode, usize); 9] = [
    (KeyCode::Digit1, KeyCode::Numpad1, 0),
    (KeyCode::Digit2, KeyCode::Numpad2, 1),
    (KeyCode::Digit3, KeyCode::Numpad3, 2),
    (KeyCode::Digit4, KeyCode::Numpad4, 3),
    (KeyCode::Digit5, KeyCode::Numpad5, 4),
    (KeyCode::Digit6, KeyCode::Numpad6, 5),
    (KeyCode::Digit7, KeyCode::Numpad7, 6),
    (KeyCode::Digit8, KeyCode::Numpad8, 7),
    (KeyCode::Digit9, KeyCode::Numpad9, 8),
];

pub fn get_just_pressed_digit(kb_input: Res<ButtonInput<KeyCode>>) -> Option<usize> {
    for &(key1, key2, digit) in &KEY_MAP {
        if kb_input.just_pressed(key1) || kb_input.just_pressed(key2) {
            return Some(digit);
        }
    }

    None
}
