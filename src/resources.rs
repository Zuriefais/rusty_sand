// resources.rs
use crate::{
    enums::{CellType, CELL_COLOR},
    grid::*,
    utils::position_to_cell_coords,
};
use bevy::{prelude::*, sprite::Mesh2dHandle, utils::HashMap};
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
pub struct CellWorld {
    quadrant_i: Grid<Option<Entity>>,   // Positive row, positive col
    quadrant_ii: Grid<Option<Entity>>,  // Negative row, positive col
    quadrant_iii: Grid<Option<Entity>>, // Negative row, negative col
    quadrant_iv: Grid<Option<Entity>>,  // Positive row, negative col
    pub cell_count: usize,
}

impl CellWorld {
    pub fn default() -> Self {
        let rows = 1000;
        let cols = 1000;
        Self {
            quadrant_i: Grid::new(rows, cols),
            quadrant_ii: Grid::new(rows, cols),
            quadrant_iii: Grid::new(rows, cols),
            quadrant_iv: Grid::new(rows, cols),
            cell_count: 0,
        }
    }

    pub fn insert(&mut self, row: isize, col: isize, entity: Option<Entity>) {
        let (row_idx, col_idx) = (row.abs() as usize, col.abs() as usize);

        match (row >= 0, col >= 0) {
            (true, true) => self.quadrant_i[(row_idx, col_idx)] = entity,
            (false, true) => self.quadrant_ii[(row_idx - 1, col_idx)] = entity,
            (false, false) => self.quadrant_iii[(row_idx - 1, col_idx - 1)] = entity,
            (true, false) => self.quadrant_iv[(row_idx, col_idx - 1)] = entity,
        }
    }

    pub fn insert_if_empty(&mut self, pos: (isize, isize), entity: Entity) {
        if self.is_cell_empty(pos) {
            self.insert(pos.0 as isize, pos.1 as isize, Some(entity));
            self.cell_count += 1;
        }
    }

    pub fn is_cell_empty(&self, pos: (isize, isize)) -> bool {
        match self.get(pos.0, pos.1) {
            None => true,
            Some(_) => false, // Cell is empty (either contains None or is out of bounds)
        }
    }

    pub fn insert_by_pos_if_empty(&mut self, pos: Vec2, entity: Entity) {
        let pos = position_to_cell_coords(pos);
        self.insert_if_empty(pos, entity);
        info!("cell count {}, x: {}, y: {}", self.cell_count, pos.0, pos.1)
    }

    pub fn get(&self, row: isize, col: isize) -> Option<Entity> {
        let (row_idx, col_idx) = (row.abs() as usize, col.abs() as usize);

        match (row >= 0, col >= 0) {
            (true, true) => self.quadrant_i[(row_idx, col_idx)],
            (false, true) => self.quadrant_ii[(row_idx - 1, col_idx)],
            (false, false) => self.quadrant_iii[(row_idx - 1, col_idx - 1)],
            (true, false) => self.quadrant_iv[(row_idx, col_idx - 1)],
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

#[cfg(test)]
mod tests {
    use bevy::ecs::entity::Entity;

    use crate::resources::CellWorld;

    #[test]
    fn if_cell_world_is_empty_fn() {
        let mut cell_world = CellWorld::default();
        cell_world.insert(10, 10, Some(Entity::from_raw(10)));
        println!("Cell at (1, 1): {:?}", cell_world.get(1, 1));
        assert_eq!(true, cell_world.is_cell_empty((1, 1)));
        assert_eq!(false, cell_world.is_cell_empty((10, 10)));
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
