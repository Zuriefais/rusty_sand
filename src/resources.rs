// resources.rs
use crate::grid::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct World {
    pub grid: Grid<Entity>,
    pub cell_size: Vec3,
}

impl World {
    pub fn default() -> Self {
        World {
            grid: grid![],
            cell_size: Vec3::new(10.0, 10.0, 10.0),
        }
    }
}
