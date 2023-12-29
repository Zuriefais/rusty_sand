// components.rs
use crate::enums::CellType;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Cell {
    pub cell_type: CellType,
}

#[derive(Component)]
pub struct MainCamera;
