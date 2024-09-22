use crate::interact::events::InteractEvent;
use crate::inventory::components::Inventory;
use crate::inventory::item::Item;
use crate::utils::get_slot::get_just_pressed_digit;
use bevy::prelude::*;

pub fn select_item<T: Send + Sync + 'static>(
    mut q_inventories: Query<&mut Inventory<T>>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let new_slot = get_just_pressed_digit(kb_input);

    if let Some(new_slot) = new_slot {
        for mut inventory in q_inventories.iter_mut() {
            inventory.set_selected_slot(new_slot);
        }
    }
}

pub fn pickup_item<T: Send + Sync + Clone + 'static>(
    mut commands: Commands,
    mut event: EventReader<InteractEvent>,
    mut q_inventories: Query<&mut Inventory<T>>,
    q_items: Query<(&Item<T>, Entity)>,
) {
    for ev in event.read() {
        if let Ok((item, entity)) = q_items.get(ev.recipient)
            && let Ok(mut inventory) = q_inventories.get_mut(ev.initiator)
        {
            let added = inventory.add_item(item.0.clone());

            // Despawn the item if successfully collected
            if added {
                commands.entity(entity).despawn()
            }
        }
    }
}
