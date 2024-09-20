use crate::configuration::toml_loader::ConfigState;
use bevy::prelude::*;

pub fn load_config<T: bevy::prelude::Asset>(
    mut state: ResMut<ConfigState<T>>,
    asset_server: Res<AssetServer>,
) {
    state.handle = asset_server.load::<T>("config.toml");
}
