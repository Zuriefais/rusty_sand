// enums.rs
use bevy::prelude::*;
use bevy_enum_filter::EnumFilter;
use serde::Deserialize;

#[derive(Component, EnumFilter, Debug, Deserialize, Clone)]
pub enum CellPhysicsType {
    Sand,
    Fluid,
    Tap(String),
    Solid,
}

pub const CELL_SIZE: Vec3 = Vec3::new(10.0, 10.0, 10.0);

pub const CHUNK_SIZE: bevy::prelude::IVec2 = IVec2::new(100, 100);

pub const CHUNK_SIZE_LEN: usize = (CHUNK_SIZE.x * CHUNK_SIZE.y) as usize;
