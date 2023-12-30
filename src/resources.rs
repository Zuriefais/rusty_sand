// resources.rs
use crate::{
    enums::{CellType, CELL_COLOR},
    grid::*,
    utils::position_to_cell_coords,
};
use bevy::{prelude::*, sprite::Mesh2dHandle, utils::HashMap};
use strum::IntoEnumIterator;

#[derive(Resource)]
pub struct EguiHoverState {
    pub is_hovered: bool,
}

impl EguiHoverState {
    pub fn default() -> Self {
        Self { is_hovered: true }
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

    pub fn insert(&mut self, pos: (usize, usize), entity: Entity) {
        if let Some(cell) = self.grid.get_mut(pos.0, pos.1) {
            *cell = Some(entity);
        }
    }

    pub fn insert_if_empty(&mut self, pos: (usize, usize), entity: Entity) {
        if self.is_cell_empty(pos) {
            self.insert(pos, entity);
            self.cell_count += 1;
        }
    }

    pub fn is_cell_empty(&self, pos: (usize, usize)) -> bool {
        match self.grid.get(pos.0, pos.1) {
            Some(&Some(_)) => false, // Cell is not empty, contains an Entity
            _ => true,               // Cell is empty (either contains None or is out of bounds)
        }
    }

    pub fn insert_by_pos_if_empty(&mut self, pos: Vec2, entity: Entity) {
        let pos = position_to_cell_coords(pos);
        self.insert_if_empty(pos, entity);
        info!("cell count {}, x: {}, y: {}", self.cell_count, pos.0, pos.1)
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

#[cfg(test)]
mod tests {
    use crate::resources::CellWorld;

    #[test]
    fn if_cell_world_is_empty_fn() {
        let cell_world = CellWorld::default();
        assert_eq!(true, cell_world.is_cell_empty((1, 1)));
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
