use bevy::prelude::*;

#[derive(Event)]
pub struct SpawnCellEvent {
    pub pos: Vec2,
}
