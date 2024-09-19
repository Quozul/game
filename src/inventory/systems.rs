use crate::inventory::components::Inventory;
use crate::inventory::item::Item;
use crate::physics::events::CollisionEvent;
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

pub fn collect_item<T: Send + Sync + Clone + 'static + std::fmt::Debug>(
    mut commands: Commands,
    mut event: EventReader<CollisionEvent>,
    mut q_inventories: Query<&mut Inventory<T>>,
    q_items: Query<&Item<T>>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    for ev in event.read() {
        if let Some(item) = ev.get_from_query(&q_items) {
            if let Some(mut inventory) = ev.get_mut_from_query(&mut q_inventories) {
                if kb_input.just_pressed(KeyCode::KeyE) {
                    inventory.add_item(item.0.clone());

                    // Despawn the item if successfully collected
                    if let Some(entity) = ev.contains(&q_items) {
                        commands.entity(entity).despawn()
                    }
                }
            }
        }
    }
}
