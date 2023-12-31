use std::ops::Range;

use bevy::prelude::*;
use grid::Grid;

use crate::utils::position_to_cell_coords;

#[derive(Resource)]
pub struct CellWorld {
    quadrant_i: Grid<Option<Entity>>,   // Positive row, positive col
    quadrant_ii: Grid<Option<Entity>>,  // Negative row, positive col
    quadrant_iii: Grid<Option<Entity>>, // Negative row, negative col
    quadrant_iv: Grid<Option<Entity>>,  // Positive row, negative col
    pub cell_count: usize,
    pub size_rows: Range<isize>,
    pub size_cols: Range<isize>,
}

impl CellWorld {
    pub fn default() -> Self {
        let rows = 10000;
        let cols = 10000;
        Self {
            quadrant_i: Grid::new(rows, cols),
            quadrant_ii: Grid::new(rows, cols),
            quadrant_iii: Grid::new(rows, cols),
            quadrant_iv: Grid::new(rows, cols),
            cell_count: 0,
            size_rows: -(rows as isize)..rows as isize,
            size_cols: -(cols as isize)..cols as isize,
        }
    }

    pub fn insert(&mut self, row: isize, col: isize, entity: Option<Entity>) {
        if self.check_bounds((row, col)) {
            return;
        }
        let (row_idx, col_idx) = (row.abs() as usize, col.abs() as usize);

        match (row >= 0, col >= 0) {
            (true, true) => self.quadrant_i[(row_idx, col_idx)] = entity,
            (false, true) => self.quadrant_ii[(row_idx - 1, col_idx)] = entity,
            (false, false) => self.quadrant_iii[(row_idx - 1, col_idx - 1)] = entity,
            (true, false) => self.quadrant_iv[(row_idx, col_idx - 1)] = entity,
        }
        if entity.is_some() {
            self.cell_count += 1
        } else {
            self.cell_count -= 1;
        }
    }

    pub fn insert_if_empty(&mut self, pos: (isize, isize), entity: Entity) {
        if self.is_cell_empty(pos) {
            self.insert(pos.0, pos.1, Some(entity));
        }
    }

    pub fn is_cell_empty(&self, pos: (isize, isize)) -> bool {
        if self.check_bounds(pos) {
            return true;
        }
        match self.get(pos.0, pos.1) {
            None => true,
            Some(_) => false,
        }
    }

    pub fn insert_by_pos_if_empty(&mut self, pos: Vec2, entity: Entity) {
        let pos = position_to_cell_coords(pos);
        self.insert_if_empty(pos, entity);
        info!("cell count {}, x: {}, y: {}", self.cell_count, pos.0, pos.1)
    }

    pub fn get(&self, row: isize, col: isize) -> Option<Entity> {
        if self.check_bounds((row, col)) {
            return None;
        }
        let (row_idx, col_idx) = (row.abs() as usize, col.abs() as usize);

        match (row >= 0, col >= 0) {
            (true, true) => self.quadrant_i[(row_idx, col_idx)],
            (false, true) => self.quadrant_ii[(row_idx - 1, col_idx)],
            (false, false) => self.quadrant_iii[(row_idx - 1, col_idx - 1)],
            (true, false) => self.quadrant_iv[(row_idx, col_idx - 1)],
        }
    }

    pub fn check_bounds(&self, pos: (isize, isize)) -> bool {
        if !(self.size_rows.contains(&pos.0) && self.size_cols.contains(&pos.1)) {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::entity::Entity;

    use crate::resources::cell_world::CellWorld;

    #[test]
    fn if_cell_world_is_empty_fn() {
        let mut cell_world = CellWorld::default();
        cell_world.insert(10, 10, Some(Entity::from_raw(10)));
        println!("Cell at (1, 1): {:?}", cell_world.get(1, 1));
        assert_eq!(true, cell_world.is_cell_empty((1, 1)));
        assert_eq!(false, cell_world.is_cell_empty((10, 10)));
    }

    #[test]
    fn col_and_row_test() {
        let world = CellWorld::default();

        let row = 1001;
        let col = -1700;

        assert_eq!(
            false,
            world.size_rows.contains(&row) && world.size_cols.contains(&col)
        )
    }
}
