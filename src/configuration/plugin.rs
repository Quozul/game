use crate::configuration::setup::load_config;
use crate::configuration::toml_loader::{ConfigLoader, ConfigState};
use bevy::prelude::*;
use std::marker::PhantomData;

pub struct ConfigurationPlugin<T> {
    _marker: PhantomData<T>,
}

impl<T> ConfigurationPlugin<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T> Plugin for ConfigurationPlugin<T>
where
    T: 'static
        + bevy::prelude::Asset
        + for<'de> serde::Deserialize<'de>
        + Default
        + std::fmt::Debug,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<T>()
            .init_resource::<ConfigState<T>>()
            .register_asset_loader(ConfigLoader::<T>::default())
            .add_systems(Startup, load_config::<T>);
    }
}
