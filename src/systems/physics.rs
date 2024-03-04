
use bevy::prelude::*;

use crate::enums::CellPhysicsType::*;
use crate::resources::{
    cell_world::{CellWorld, Chunk},
    CellAssets,
};

pub fn physics(mut world: ResMut<CellWorld>, assets: Res<CellAssets>) {
    for (chunk_pos, chunk) in world.chunks.iter_mut() {
        // let mut chunk_below_option = world.get_mut_chunk(chunk_pos.clone() + IVec2{x:0, y:1});
        // let mut chunk_below = match chunk_below_option {
        //     Some(chunk_below) => chunk_below,
        //     None => {
        //         *chunk_below_option = Chunk::default()
        //     },
        // }
        let mut to_swap_list = vec![];
        let mut to_move_list = vec![];
        let mut to_insert_list = vec![];

        for i in 0..chunk.cells.len() - 1 {
            match chunk.cells[i] {
                Some(cell) => match assets.assets_physics_behavior_vec.get(cell.0) {
                    Some(behavior) => match behavior {
                        Sand => {
                            sand_physics(i, &chunk, &mut to_swap_list, &mut to_move_list);
                        }
                        Fluid => {
                            fluid_physics(i, &chunk, &mut to_swap_list, &mut to_move_list);
                        }
                        Tap(to_spawn) => {
                            tap_physics(&mut to_insert_list, i, chunk, to_spawn, &assets);
                        }
                        Solid => {}
                    },
                    None => {}
                },
                None => {}
            }
        }

        for to_swap in to_swap_list {
            chunk.cells.swap(to_swap.0, to_swap.1);
        }
        for to_move in to_move_list {
            if let Some(cell) = chunk.cells.get_mut(to_move.1) {
                // Use get_mut for safe mutation
                if let Some(cell) = cell {
                    // Use get_mut for safe mutation
                    cell.1 = cell.1 + to_move.0; // Explicit assignment
                }
            }
        }
        for to_insert in to_insert_list {
            if let Some(cell) = chunk.cells.get_mut(to_insert.0) {
                *cell = Some(to_insert.1);
            }
        }
    }
}

fn sand_physics(
    i: usize,
    chunk: &Chunk,
    to_swap_list: &mut Vec<(usize, usize)>,
    to_move_list: &mut Vec<(Vec2, usize)>,
) {
    let pos = Chunk::vec_index_to_ivec(i).unwrap();

    {
        if let Some(pos_below) = Chunk::get_index_below(i) {
            if chunk.cells[pos_below].is_none() {
                to_swap_list.push((i, pos_below));
                return;
            }
        } else {
            return;
        }
    };

    let is_none_below_left = get_is_none_by_offset(chunk, pos, IVec2 { x: -1, y: -1 });

    let is_none_below_right = get_is_none_by_offset(chunk, pos, IVec2 { x: 1, y: -1 });

    move_if_none(to_swap_list, is_none_below_left, is_none_below_right, i);
}

fn tap_physics(
    to_insert_list: &mut Vec<(usize, (usize, Vec2))>,
    i: usize,
    chunk: &Chunk,
    to_spawn: &String,
    assets: &CellAssets,
) {
    if let Some(cell_below_index) = Chunk::get_index_below(i) {
        if chunk.cells[cell_below_index].is_none() {
            if let Some(asset_id) = assets.get_index_by_name(to_spawn.to_string()) {
                to_insert_list.push((cell_below_index, (asset_id, Vec2::ZERO)))
            }
        }
    };
}

fn fluid_physics(
    i: usize,
    chunk: &Chunk,
    to_swap_list: &mut Vec<(usize, usize)>,
    to_move_list: &mut Vec<(Vec2, usize)>,
) {
    let pos = Chunk::vec_index_to_ivec(i).unwrap();

    {
        if let Some(pos_below) = Chunk::get_index_below(i) {
            if chunk.cells[pos_below].is_none() {
                to_swap_list.push((i, pos_below));
                return;
            }
        } else {
            return;
        }
    };

    let is_none_below_left = get_is_none_by_offset(chunk, pos, IVec2 { x: -1, y: -1 });

    let is_none_below_right = get_is_none_by_offset(chunk, pos, IVec2 { x: 1, y: -1 });

    move_if_none(to_swap_list, is_none_below_left, is_none_below_right, i);

    let is_none_left =  get_is_none_by_offset(chunk, pos, IVec2 { x: -1, y: 0 });

    let is_none_right = get_is_none_by_offset(chunk, pos, IVec2 { x: 1, y: 0 });

    move_if_none(to_swap_list, is_none_left, is_none_right, i)
}

fn get_is_none_by_offset(chunk: &Chunk, pos: IVec2, offset: IVec2) -> Option<usize> {
    let mut pos_offset = pos;
        pos_offset+=offset;

        if let Some(cell) = Chunk::ivec_to_vec_index(pos_offset) {
            if chunk.cells[cell].is_none() {
                Some(cell)
            } else {
                None
            }
        } else {
            None
        }
}

fn move_if_none(to_swap_list: &mut Vec<(usize, usize)>, is_none: Option<usize>, is_none1: Option<usize>, i: usize) {
    match (is_none, is_none1) {
        (None, None) => {
            
        }
        (None, Some(cell)) => {to_swap_list.push((cell, i)); return;},
        (Some(cell), None) => {to_swap_list.push((cell, i)); return;},
        (Some(cell), Some(cell2)) => {
            if fastrand::bool() {
                to_swap_list.push((cell, i))
            } else {
                to_swap_list.push((cell2, i))
            }
            return;
        }
    }
}
