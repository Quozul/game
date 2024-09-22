use bevy::prelude::*;

#[derive(Component)]
pub struct Inventory<T> {
    contents: Vec<T>,
    selected_slot: usize,
    capacity: usize,
}

impl<T> Inventory<T> {
    pub fn with_contents(contents: Vec<T>, capacity: usize) -> Self {
        Self {
            contents,
            selected_slot: 0,
            capacity,
        }
    }

    pub fn get_selected_item(&self) -> Option<&T> {
        let index = self.selected_slot;
        self.contents.get(index)
    }

    pub fn set_selected_slot(&mut self, new_index: usize) {
        self.selected_slot = new_index
    }

    pub fn add_item(&mut self, item: T) -> bool {
        if self.contents.len() < self.capacity {
            self.contents.push(item);
            self.selected_slot = self.contents.len() - 1;
            true
        } else {
            false
        }
    }

    pub fn remove_selected_item(&mut self) -> Option<T> {
        if self.selected_slot >= self.contents.len() {
            None
        } else {
            let removed = self.contents.remove(self.selected_slot);
            // Adjust the slot
            if self.selected_slot >= self.contents.len() {
                self.selected_slot = self.contents.len().checked_sub(1).unwrap_or_default();
            }
            Some(removed)
        }
    }
}
