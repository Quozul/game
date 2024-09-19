use bevy::prelude::*;

#[derive(Component)]
pub struct Inventory<T> {
    contents: Vec<T>,
    selected_slot: usize,
}

impl<T> Inventory<T> {
    pub fn with_contents(contents: Vec<T>) -> Self {
        Self {
            contents,
            selected_slot: 0,
        }
    }

    pub fn get_selected_item(&self) -> Option<&T> {
        let index = self.selected_slot;
        self.contents.get(index)
    }

    pub fn set_selected_slot(&mut self, new_index: usize) {
        self.selected_slot = new_index
    }

    pub fn add_item(&mut self, item: T) {
        self.contents.push(item);
    }

    pub fn remove_item(&mut self, index: usize) -> T {
        self.contents.remove(index)
    }
}
