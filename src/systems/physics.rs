use crate::enums::cell_physics_type_filters;
use crate::resources::CellAssets;
use crate::{
    enums::CELL_SIZE,
    events::SpawnCellEvent,
    resources::{cell_world::CellWorld, SimulateWorldState},
    utils::position_to_cell_coords,
};
use bevy::prelude::*;
use bevy_enum_filter::Enum;

pub fn sand_physics(
    mut query: Query<(Entity, &mut Transform), With<Enum!(CellPhysicsType::Sand)>>,
    mut cell_world: ResMut<CellWorld>,
    state: Res<SimulateWorldState>,
) {
    if !state.is_simulating {
        return;
    }

    for (entity, mut transform) in query.iter_mut() {
        let below_x = (transform.translation.x / CELL_SIZE.x).floor() as isize;
        let below_y = ((transform.translation.y - CELL_SIZE.y) / CELL_SIZE.y).floor() as isize;
        if cell_world.get(below_x, below_y).is_none() {
            transform.translation.y -= CELL_SIZE.y;

            cell_world.insert(below_x, below_y, Some(entity));
            cell_world.insert(below_x, below_y + 1, None);
        } else if cell_world.get(below_x - 1, below_y).is_none()
            && !cell_world.get(below_x - 1, below_y + 1).is_some()
        {
            transform.translation.y -= CELL_SIZE.y;
            transform.translation.x -= CELL_SIZE.x;

            cell_world.insert(below_x - 1, below_y, Some(entity));
            cell_world.insert(below_x, below_y + 1, None);
        } else if cell_world.get(below_x + 1, below_y).is_none()
            && !cell_world.get(below_x + 1, below_y + 1).is_some()
        {
            transform.translation.y -= CELL_SIZE.y;
            transform.translation.x += CELL_SIZE.x;

            cell_world.insert(below_x + 1, below_y, Some(entity));
            cell_world.insert(below_x, below_y + 1, None);
        }
    }
}

pub fn fluid_physics(
    mut query: Query<(Entity, &mut Transform), With<Enum!(CellPhysicsType::Fluid)>>,
    mut cell_world: ResMut<CellWorld>,
    state: Res<SimulateWorldState>,
) {
    if !state.is_simulating {
        return;
    }

    for (entity, mut transform) in query.iter_mut() {
        let below_x = (transform.translation.x / CELL_SIZE.x).floor() as isize;
        let below_y = ((transform.translation.y - CELL_SIZE.y) / CELL_SIZE.y).floor() as isize;
        if cell_world.get(below_x, below_y).is_none() {
            transform.translation.y -= CELL_SIZE.y;

            cell_world.insert(below_x, below_y, Some(entity));
            cell_world.insert(below_x, below_y + 1, None);
        } else if cell_world.get(below_x - 1, below_y).is_none()
            && !cell_world.get(below_x - 1, below_y + 1).is_some()
        {
            transform.translation.y -= CELL_SIZE.y;
            transform.translation.x -= CELL_SIZE.x;

            cell_world.insert(below_x - 1, below_y, Some(entity));
            cell_world.insert(below_x, below_y + 1, None);
        } else if cell_world.get(below_x + 1, below_y).is_none()
            && !cell_world.get(below_x + 1, below_y + 1).is_some()
        {
            transform.translation.y -= CELL_SIZE.y;
            transform.translation.x += CELL_SIZE.x;

            cell_world.insert(below_x + 1, below_y, Some(entity));
            cell_world.insert(below_x, below_y + 1, None);
        } else if cell_world.get(below_x - 1, below_y + 1).is_none() {
            transform.translation.x -= CELL_SIZE.x;

            cell_world.insert(below_x - 1, below_y + 1, Some(entity));
            cell_world.insert(below_x, below_y + 1, None);
        } else if cell_world.get(below_x + 1, below_y + 1).is_none() {
            transform.translation.x += CELL_SIZE.x;

            cell_world.insert(below_x + 1, below_y + 1, Some(entity));
            cell_world.insert(below_x, below_y + 1, None);
        }
    }
}

pub fn blood_stone_physics(
    mut query: Query<&mut Transform, With<Enum!(CellPhysicsType::BloodStone)>>,
    cell_world: ResMut<CellWorld>,
    mut ev_spawn_cell: EventWriter<SpawnCellEvent>,
    state: Res<SimulateWorldState>,
    cell_assets: Res<CellAssets>,
) {
    if !state.is_simulating {
        return;
    }

    for transform in query.iter_mut() {
        let mut pos = transform.translation;
        pos.y -= CELL_SIZE.y;

        let grid_pos = position_to_cell_coords(pos);
        if !cell_world.is_cell_empty(grid_pos) {
            continue;
        }
        ev_spawn_cell.send(SpawnCellEvent {
            pos: Vec2::new(pos.x, pos.y),
            cell_type: cell_assets.handles["blood"].clone(),
        });
    }
}
