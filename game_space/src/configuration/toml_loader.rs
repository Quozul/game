use bevy::asset::*;
use bevy::prelude::Resource;
use std::marker::PhantomData;
use thiserror::Error;

#[derive(Resource, Default)]
pub struct ConfigState<T: bevy::prelude::Asset> {
    pub handle: Handle<T>,
    pub used: bool,
}

#[derive(Default)]
pub struct ConfigLoader<T> {
    _marker: PhantomData<T>,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ConfigLoaderError {
    #[error("could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("could not parse toml: {0}")]
    RonSpannedError(#[from] toml::de::Error),
}

impl<T> AssetLoader for ConfigLoader<T>
where
    T: 'static + bevy::prelude::Asset + for<'de> serde::Deserialize<'de>,
{
    type Asset = T;
    type Settings = ();
    type Error = ConfigLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut io::Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf).await?;
        let result = toml::from_str::<Self::Asset>(&buf)?;
        Ok(result)
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}
