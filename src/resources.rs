pub mod cell_world;

use crate::assets::CellAsset;
use bevy::{prelude::*, sprite::Mesh2dHandle, utils::hashbrown::HashMap};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct SimulateWorldState {
    pub is_simulating: bool,
}

impl SimulateWorldState {
    pub fn default() -> Self {
        Self {
            is_simulating: true,
        }
    }
}

#[derive(Resource)]
pub struct EguiHoverState {
    pub is_hovered: bool,
}

impl EguiHoverState {
    pub fn default() -> Self {
        Self { is_hovered: false }
    }
}

#[derive(Resource, Default, Clone)]
pub struct CellAssets {
    pub assets: Vec<CellAsset>,
    pub assets_ids_map: HashMap<String, usize>,
    pub assets_color_vec: Vec<Color>,
}

impl CellAssets {
    pub fn get_by_name(&self, name: String) -> Option<CellAsset> {
        if let Some(handle_index) = self.assets_ids_map.get(&name) {
            return self.assets.get(handle_index.clone()).cloned();
        }

        None
    }

    pub fn get_index_by_name(&self, name: String) -> Option<usize> {
        self.assets_ids_map.get(&name).copied()
    }

    pub fn get(&self, i: usize) -> Option<CellAsset> {
        self.assets.get(i).cloned()
    }

    pub fn get_color(&self, i: usize) -> Option<Color> {
        Some(self.assets_color_vec[i])
    }

    pub fn get_color_by_name(&self, name: String) -> Option<Color> {
        if let Some(handle_index) = self.assets_ids_map.get(&name) {
            return Some(self.assets_color_vec[*handle_index]);
        }
        None
    }

    pub fn add(&mut self, asset: CellAsset) {
        self.assets.push(asset.clone());
        self.assets_color_vec.push(asset.color);
        self.assets_ids_map
            .insert(asset.name, self.assets.len() - 1);
    }

    pub fn remove() {}

    pub fn get_last_index(self) -> usize {
        self.assets.len() - 1
    }
}

#[derive(Resource)]
pub struct CursorPosition {
    pub pos: Vec2,
}

impl CursorPosition {
    pub fn default() -> Self {
        Self { pos: Vec2::ZERO }
    }
}

#[derive(Resource)]
pub struct CellMesh {
    pub mesh: Mesh2dHandle,
}

impl CellMesh {
    pub fn from_world(mut meshes: ResMut<Assets<Mesh>>) -> Self {
        let mesh = meshes.add(Mesh::from(shape::Quad::default())).into();

        CellMesh { mesh }
    }
}

#[derive(Resource)]
pub struct CellTypeToSpawn {
    pub selected: Option<Selected>,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Selected {
    pub name: String,
    pub handle: usize,
}

impl CellTypeToSpawn {
    pub fn default() -> Self {
        Self { selected: None }
    }
}
