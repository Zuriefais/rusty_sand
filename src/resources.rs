// resources.rs
use crate::{
    enums::{CellType, CELL_COLOR},
    grid::*,
};
use bevy::{prelude::*, utils::HashMap, sprite::Mesh2dHandle};
use strum::IntoEnumIterator;

#[derive(Resource)]
pub struct CellWorld {
    pub grid: Grid<Entity>,
    pub cell_size: Vec3,
}

impl CellWorld {
    pub fn default() -> Self {
        CellWorld {
            grid: grid![],
            cell_size: Vec3::new(10.0, 10.0, 10.0),
        }
    }
}

#[derive(Resource)]
pub struct SandMaterials {
    pub materials: Vec<Handle<ColorMaterial>>,
    pub color_ids: HashMap<CellType, usize>,
}

impl SandMaterials {
    pub fn from_world(mut materials: ResMut<Assets<ColorMaterial>>,) -> Self {
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

#[derive(Resource)]
pub struct CellMesh {
    pub mesh: Mesh2dHandle,
}

impl CellMesh {
    pub fn from_world(mut meshes: ResMut<Assets<Mesh>>,) -> Self {
        let mesh = meshes.add(Mesh::from(shape::Quad::default())).into();

        CellMesh { mesh }
    }
}
