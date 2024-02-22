use crate::enums::CellPhysicsType;
use bevy::asset::io::Reader;
use bevy::asset::{AsyncReadExt, LoadedAsset};
use bevy::reflect::TypeUuid;
use bevy::utils::thiserror;
use bevy::{
    asset::{AssetLoader, LoadContext},
    prelude::*,
    utils::BoxedFuture,
};
use bevy_inspector_egui::InspectorOptions;
use serde::{Deserialize, Deserializer};
use std::str;
use thiserror::Error;

#[uuid = "5b68f25a-835d-45f2-855d-94613a2da2fd"]
#[derive(Asset, TypePath, Debug, Deserialize, InspectorOptions, TypeUuid)]
pub struct CellAsset {
    pub cell_physics_behavior: CellPhysicsType,
    #[serde(deserialize_with = "hex_to_color")]
    pub color: Color,
    pub cell_type_name: String,
    #[serde(default = "default_i32")]
    pub density: i32,
    #[serde(skip)]
    pub material: Handle<ColorMaterial>,
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
    type Asset = CellAsset;
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
            let mut custom_asset = ron::de::from_bytes::<CellAsset>(&bytes)?;
            let color_material = LoadedAsset::from(ColorMaterial::from(custom_asset.color));
            custom_asset.material =
                load_context.add_loaded_labeled_asset("Color material", color_material);
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
