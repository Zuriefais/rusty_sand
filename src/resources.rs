// resources.rs
use crate::grid::*;
use bevy::prelude::*;

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
