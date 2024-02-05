use crate::enums::{cell_physics_type_filters, CellPhysicsType};
use crate::resources::CellAssets;
use crate::utils::ivec2_to_vec3;
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
    let mut pos: IVec2;
    let mut pos_below: IVec2;
    let mut pos_below_right: IVec2;
    let mut pos_below_left: IVec2;
    for (entity, mut transform) in query.iter_mut() {
        pos = position_to_cell_coords(transform.translation);

        pos_below = pos;
        pos_below.y -= 1;

        pos_below_right = pos_below;
        pos_below_right.x += 1;

        pos_below_left = pos_below;
        pos_below_left.x -= 1;

        if cell_world.get(pos_below).is_none() {
            transform.translation.y -= CELL_SIZE.y;

            cell_world.insert(pos, Some(entity));
            pos.y += 1;
            cell_world.insert(pos, None);
        } else if cell_world.get(pos_below_left).is_none()
            && cell_world.get(pos_below_right).is_none()
        {
            let right_or_left = fastrand::bool();

            let pos_to_move = match right_or_left {
                true => pos_below_right,
                false => pos_below_left,
            };

            cell_world.insert(pos, None);
            cell_world.insert(pos_to_move, Some(entity));

            transform.translation = ivec2_to_vec3(pos_to_move);
        } else if cell_world.get(pos_below_left).is_none()
            && !cell_world.get(pos_below_right).is_none()
        {
            transform.translation.y -= CELL_SIZE.y;
            transform.translation.x -= CELL_SIZE.x;

            cell_world.insert(pos, None);
            cell_world.insert(pos_below_left, Some(entity));
        } else if !cell_world.get(pos_below_left).is_none()
            && cell_world.get(pos_below_right).is_none()
        {
            transform.translation.y -= CELL_SIZE.y;
            transform.translation.x += CELL_SIZE.x;

            cell_world.insert(pos, None);
            cell_world.insert(pos_below_left, Some(entity));
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
    let mut pos: IVec2;
    let mut pos_right: IVec2;
    let mut pos_left: IVec2;
    let mut pos_below: IVec2;
    let mut pos_below_right: IVec2;
    let mut pos_below_left: IVec2;
    for (entity, mut transform) in query.iter_mut() {
        pos = position_to_cell_coords(transform.translation);

        pos_left = pos;
        pos_left.x -= 1;

        pos_right = pos;
        pos_right.x += 1;

        pos_below = pos;
        pos_below.y -= 1;

        pos_below_right = pos_below;
        pos_below_right.x += 1;

        pos_below_left = pos_below;
        pos_below_left.x -= 1;

        if cell_world.get(pos_below).is_none() {
            transform.translation.y -= CELL_SIZE.y;

            cell_world.insert(pos, Some(entity));
            pos.y += 1;
            cell_world.insert(pos, None);
        } else if cell_world.get(pos_below_left).is_none()
            && cell_world.get(pos_below_right).is_none()
        {
            let right_or_left = fastrand::bool();

            let pos_to_move = match right_or_left {
                true => pos_below_right,
                false => pos_below_left,
            };

            cell_world.insert(pos, None);
            cell_world.insert(pos_to_move, Some(entity));

            transform.translation = ivec2_to_vec3(pos_to_move);
        } else if cell_world.get(pos_left).is_none() && cell_world.get(pos_right).is_none() {
            let right_or_left = fastrand::bool();

            let pos_to_move = match right_or_left {
                true => pos_right,
                false => pos_left,
            };

            cell_world.insert(pos, None);
            cell_world.insert(pos_to_move, Some(entity));

            transform.translation = ivec2_to_vec3(pos_to_move);
        } else if cell_world.get(pos_below_left).is_none()
            && !cell_world.get(pos_below_right).is_none()
        {
            transform.translation.y -= CELL_SIZE.y;
            transform.translation.x -= CELL_SIZE.x;

            cell_world.insert(pos, None);
            cell_world.insert(pos_below_left, Some(entity));
        } else if !cell_world.get(pos_below_left).is_none()
            && cell_world.get(pos_below_right).is_none()
        {
            transform.translation.y -= CELL_SIZE.y;
            transform.translation.x += CELL_SIZE.x;

            cell_world.insert(pos, None);
            cell_world.insert(pos_below_left, Some(entity));
        } else if cell_world.get(pos_left).is_none() && !cell_world.get(pos_right).is_none() {
            transform.translation.x -= CELL_SIZE.x;

            cell_world.insert(pos, None);
            cell_world.insert(pos_below_left, Some(entity));
        } else if !cell_world.get(pos_left).is_none() && cell_world.get(pos_right).is_none() {
            transform.translation.x += CELL_SIZE.x;

            cell_world.insert(pos, None);
            cell_world.insert(pos_below_left, Some(entity));
        }
    }
}

pub fn tap_physics(
    mut query: Query<(&mut Transform, &CellPhysicsType), With<Enum!(CellPhysicsType::Tap)>>,
    cell_world: ResMut<CellWorld>,
    mut ev_spawn_cell: EventWriter<SpawnCellEvent>,
    state: Res<SimulateWorldState>,
    cell_assets: Res<CellAssets>,
) {
    if !state.is_simulating {
        return;
    }

    for (transform, tap) in query.iter_mut() {
        match tap {
            CellPhysicsType::Tap(spawn_type) => {
                let mut pos = transform.translation;
                pos.y -= CELL_SIZE.y;

                let grid_pos = position_to_cell_coords(pos);
                if !cell_world.is_cell_empty(grid_pos) {
                    continue;
                }
                ev_spawn_cell.send(SpawnCellEvent {
                    pos: Vec2::new(pos.x, pos.y),
                    cell_type: cell_assets.handles[spawn_type].clone(),
                });
            }
            _ => return,
        }
    }
}
