use bevy::prelude::*;

use crate::enums::CellType;

#[derive(Event)]
pub struct SpawnCellEvent {
    pub pos: Vec2,
    pub cell_type: CellType,
}

#[derive(Event)]
pub struct RemoveCellEvent {
    pub pos: (isize, isize),
}
