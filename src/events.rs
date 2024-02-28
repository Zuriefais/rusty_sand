use bevy::prelude::*;


#[derive(Event)]
pub struct SpawnCellEvent {
    pub pos: Vec2,
    pub cell_type: usize,
}

#[derive(Event)]
pub struct RemoveCellEvent {
    pub pos: IVec2,
}
