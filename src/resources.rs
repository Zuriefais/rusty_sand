// resources.rs
use crate::{
    enums::{CellType, CELL_COLOR, CELL_SIZE},
    grid::*,
};
use bevy::{prelude::*, sprite::Mesh2dHandle, utils::HashMap};
use bevy_inspector_egui::InspectorOptions;
use strum::IntoEnumIterator;

#[derive(Resource, Reflect, InspectorOptions, Default)]
pub struct HandleInputOnMouse {
    pub handle: bool,
}

impl HandleInputOnMouse {
    pub fn default() -> Self {
        HandleInputOnMouse { handle: true }
    }
}

#[derive(Resource)]
pub struct CellWorld {
    pub grid: Grid<Option<Entity>>,
    pub cell_count: i32,
}

impl CellWorld {
    pub fn default() -> Self {
        CellWorld {
            grid: grid![],
            cell_count: 0,
        }
    }

    pub fn insert(&mut self, x: usize, y: usize, entity: Entity) {
        if let Some(cell) = self.grid.get_mut(x, y) {
            *cell = Some(entity);
        }
    }

    pub fn insert_if_empty(&mut self, x: usize, y: usize, entity: Entity) {
        if let Some(cell) = self.grid.get_mut(x, y) {
            // Only insert the entity if the cell is empty (None)
            if cell.is_none() {
                *cell = Some(entity);
                self.cell_count += 1;
                info!("cell count {}", self.cell_count)
            }
        }
    }

    pub fn insert_by_pos_if_empty(&mut self, pos: Vec2, entity: Entity) {
        // Convert the Vec2 position to grid coordinates
        let x = (pos.x / CELL_SIZE.x as f32).floor() as usize;
        let y = (pos.y / CELL_SIZE.x as f32).floor() as usize;

        // Insert the entity into the grid
        self.insert_if_empty(x, y, entity);
        self.cell_count += 1;
        info!("cell count {}", self.cell_count)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Entity> {
        self.grid.get(x, y).and_then(|cell| *cell)
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
