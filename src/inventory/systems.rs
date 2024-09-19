use crate::inventory::components::Inventory;
use bevy::prelude::*;

pub fn select_item<T: Send + Sync + 'static>(
    mut q_inventories: Query<&mut Inventory<T>>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let new_slot = if kb_input.just_pressed(KeyCode::Digit1) {
        Some(0)
    } else if kb_input.just_pressed(KeyCode::Digit2) {
        Some(1)
    } else if kb_input.just_pressed(KeyCode::Digit3) {
        Some(2)
    } else {
        None
    };

    if let Some(new_slot) = new_slot {
        for mut inventory in q_inventories.iter_mut() {
            inventory.set_selected_slot(new_slot);
        }
    }
}
