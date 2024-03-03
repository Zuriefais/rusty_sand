use bevy::{ecs::world, prelude::*};

use crate::enums::{CellPhysicsType::*, CELL_SIZE};
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

        for i in 0..chunk.cells.len() - 1 {
            match chunk.cells[i] {
                Some(cell) => {
                    match assets.assets_physics_behavior_vec.get(cell.0) {
                        Some(behavior) => {
                            match behavior {
                                Sand => {
                                    sand_physics(i, &chunk, &mut to_swap_list, &mut to_move_list);
                                },
                                Fluid => {

                                },
                                Tap(fluid) => {},
                                Solid => {},
                            }
                        }
                        None => {},
                    }
                },
                None => {},
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
                    info!("{:?}, {:?}", to_move, chunk.cells[to_move.1]);
                }
            }
        }
    }
}

pub fn sand_physics(
    i: usize,
    chunk: &Chunk,
    to_swap_list: &mut Vec<(usize, usize)>,
    to_move_list: &mut Vec<(Vec2, usize)>,
) {
    let cell_option = (&chunk.cells[i], i);
    let cell_below_option = match Chunk::get_index_below(i) {
        Some(i) => (Some(&chunk.cells[i]), i),
        None => (None, 0),
    };
    if let Some(cell_below) = cell_below_option.0 {
        if let Some(cell) = cell_option.0 {
            if cell_below.is_none() {
                // to_swap_list.push((cell_below_option.1, cell_option.1));
                if cell.1.y < -0.5 {
                    to_swap_list.push((cell_option.1, cell_below_option.1));
                    to_move_list.push((Vec2 { x: 0.0, y: -0.5 }, cell_option.1));
                    info!("{:?}", Chunk::vec_index_to_ivec(cell_option.1));
                } else {
                    to_move_list.push((Vec2 { x: 0.0, y: -0.1 }, cell_option.1));
                    //info!("{:?}", cell.1);
                }
            }
        }
    }
}

pub fn fluid_physics() {}

pub fn tap_physics() {}
