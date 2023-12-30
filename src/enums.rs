// enums.rs
use bevy::{prelude::*, utils::hashbrown::HashMap};
use lazy_static::lazy_static;
use strum_macros::EnumIter;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy, EnumIter)]
pub enum CellType {
    Sand,
    Stone,
    BloodStone,
}

pub const CELL_SIZE: Vec3 = Vec3::new(10.0, 10.0, 10.0);

lazy_static! {
    pub static ref CELL_COLOR: HashMap<CellType, bevy::render::color::Color> = {
        let mut map = HashMap::new();
        map.insert(
            CellType::Sand,
            bevy::render::color::Color::hex("f6d7b0").unwrap(),
        );
        map.insert(
            CellType::Stone,
            bevy::render::color::Color::hex("4E5754").unwrap(),
        );
        map.insert(
            CellType::BloodStone,
            bevy::render::color::Color::hex("8B0000").unwrap(),
        );
        map
    };
}
