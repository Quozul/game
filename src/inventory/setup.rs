use crate::configuration::configuration::Configuration;
use crate::configuration::toml_loader::ConfigState;
use bevy::asset::{AssetServer, Assets};
use bevy::math::Vec3;
use bevy::prelude::*;

pub fn setup(
    mut state: ResMut<ConfigState<Configuration>>,
    custom_assets: Res<Assets<Configuration>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if state.used {
        return;
    }

    match custom_assets.get(&state.handle) {
        None => {
            info!("Custom Asset Not Ready");
        }
        Some(configuration) => {
            info!("{:?}", configuration);
            configuration
                .weapons
                .iter()
                .enumerate()
                .for_each(|(i, weapon)| {
                    commands.spawn(weapon.create_item_bundle(
                        &asset_server,
                        &mut texture_atlas_layouts,
                        Vec3::new(100.0 * (i + 1) as f32, 0.0, 0.0),
                    ));
                });
            state.used = true;
        }
    }
}
