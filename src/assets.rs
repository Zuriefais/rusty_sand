use crate::enums::CellPhysicsType;
use bevy::asset::io::Reader;
use bevy::asset::AsyncReadExt;
use bevy::utils::thiserror;
use bevy::{
    asset::{AssetLoader, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};
use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Deserializer};
use strum_macros::Display;
use std::str;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize, InspectorOptions, Clone)]
pub struct CellAssetToLoad {
    pub physics_behavior: CellPhysicsType,
    #[serde(deserialize_with = "hex_to_color")]
    pub color: Color,
    pub name: String,
    #[serde(default = "default_i32")]
    pub density: i32,
}

impl CellAssetToLoad {
    pub fn to_cell_asset(self) -> CellAsset {
        CellAsset { physics_behavior: self.physics_behavior, color: self.color, name: self.name, density: self.density  }
    }
}

#[derive(Clone, Debug)]
pub struct CellAsset {
    pub physics_behavior: CellPhysicsType,
    pub color: Color,
    pub name: String,
    pub density: i32,
}

fn default_i32() -> i32 {
    1 // Your default value here
}

fn hex_to_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Color::hex(s).map_err(serde::de::Error::custom)
}

#[derive(Default)]
pub struct CellAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CellAssetLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for CellAssetLoader {
    type Asset = CellAssetToLoad;
    type Settings = ();
    type Error = CellAssetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = ron::de::from_bytes::<CellAssetToLoad>(&bytes)?;
            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["cell"]
    }
}

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct ConfigAsset {
    pub cell_paths: Vec<String>,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ConfigAssetLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse TOML: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

#[derive(Default)]
pub struct ConfigAssetLoader;

impl AssetLoader for ConfigAssetLoader {
    type Asset = ConfigAsset;
    type Settings = ();
    type Error = CellAssetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = ron::de::from_bytes::<ConfigAsset>(&bytes)?;
            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["config"]
    }
}
