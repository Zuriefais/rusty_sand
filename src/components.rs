// components.rs
use crate::enums::CellType;
use bevy::prelude::*;

#[derive(Component)]
pub struct Cell {
    pub cell_type: CellType,
}

#[derive(Component)]
pub struct CursorPosition {
    pub pos: Vec2,
}

#[derive(Component)]
pub struct MainCamera;
