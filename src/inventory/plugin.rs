use crate::inventory::systems::select_item;
use bevy::prelude::*;
use std::marker::PhantomData;

pub struct InventoryPlugin<T> {
    _marker: PhantomData<T>,
}

impl<T> Plugin for InventoryPlugin<T>
where
    T: Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, select_item::<T>);
    }
}

impl<T> InventoryPlugin<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
