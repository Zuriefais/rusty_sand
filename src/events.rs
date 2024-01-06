use bevy::prelude::*;

use crate::assets::CellAsset;

#[derive(Event)]
pub struct SpawnCellEvent {
    pub pos: Vec2,
    pub cell_type: Handle<CellAsset>,
}

#[derive(Event)]
pub struct RemoveCellEvent {
    pub pos: (isize, isize),
}
