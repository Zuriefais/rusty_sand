pub mod cell_world;

use crate::enums::{CellType, CELL_COLOR};
use bevy::{prelude::*, sprite::Mesh2dHandle, utils::hashbrown::HashMap};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};
use strum::IntoEnumIterator;

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

#[derive(Resource)]
pub struct CursorPosition {
    pub pos: Vec2,
}

impl CursorPosition {
    pub fn default() -> Self {
        return Self { pos: Vec2::ZERO };
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
    pub type_to_select: CellType,
}

impl CellTypeToSpawn {
    pub fn default() -> Self {
        CellTypeToSpawn {
            type_to_select: CellType::Sand,
        }
    }
}

#[derive(Resource)]
pub struct SandMaterials {
    pub materials: Vec<Handle<ColorMaterial>>,
    pub color_ids: HashMap<CellType, usize>,
}

impl SandMaterials {
    pub fn from_world(mut materials: ResMut<Assets<ColorMaterial>>) -> Self {
        let mut sand_materials: Vec<Handle<ColorMaterial>> = vec![];
        let mut color_ids: HashMap<CellType, usize> = HashMap::new();

        for cell_type in CellType::iter() {
            sand_materials.push(materials.add(ColorMaterial::from(CELL_COLOR[&cell_type])));
            color_ids.insert(cell_type, sand_materials.len() - 1);
        }
        let sand_materials_resource: SandMaterials = SandMaterials {
            materials: sand_materials,
            color_ids,
        };

        sand_materials_resource
    }
}
