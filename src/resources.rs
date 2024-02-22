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

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct CellAssets {
    pub handles: HashMap<String, Handle<CellAsset>>,
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
    pub handle: Handle<CellAsset>,
}

impl CellTypeToSpawn {
    pub fn default() -> Self {
        Self { selected: None }
    }
}
