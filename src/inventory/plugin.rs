use crate::inventory::setup::setup;
use crate::inventory::systems::{pickup_item, select_item};
use bevy::prelude::*;
use std::marker::PhantomData;

pub struct InventoryPlugin<T> {
    _marker: PhantomData<T>,
}

impl<T> InventoryPlugin<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T> Plugin for InventoryPlugin<T>
where
    T: Send + Sync + Clone + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (setup, select_item::<T>, pickup_item::<T>));
    }
}
