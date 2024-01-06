// enums.rs
use bevy::prelude::*;
use bevy_enum_filter::EnumFilter;
use serde::Deserialize;

#[derive(Component, EnumFilter, Debug, Deserialize, Clone)]
pub enum CellPhysicsType {
    Sand,
    Fluid,
    BloodStone,
    Solid,
}

pub const CELL_SIZE: Vec3 = Vec3::new(10.0, 10.0, 10.0);
