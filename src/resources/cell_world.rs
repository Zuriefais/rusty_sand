use bevy::{prelude::*, utils::HashMap};

use crate::{
    enums::{CHUNK_SIZE, CHUNK_SIZE_LEN},
    utils::ivec2_to_vec3,
};
pub struct Chunk {
    pub cells: [Option<Entity>; CHUNK_SIZE_LEN],
    pub cell_count: usize,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            cells: [None; CHUNK_SIZE_LEN],
            cell_count: 0,
        }
    }
}

impl Chunk {
    pub fn get(&self, pos: IVec2) -> Option<Entity> {
        match Chunk::ivec_to_vec_index(pos) {
            Some(index) => self.cells[index],
            None => None,
        }
    }

    pub fn get_mut(&mut self, pos: IVec2) -> Option<&mut Option<Entity>> {
        match Chunk::ivec_to_vec_index(pos) {
            Some(index) => Some(&mut self.cells[index]),
            None => None,
        }
    }

    pub fn insert(&mut self, pos: IVec2, cell: Option<Entity>) {
        match self.get_mut(pos) {
            Some(some_cell) => {
                *some_cell = cell;
            }
            None => {}
        }
    }

    pub fn ivec_to_vec_index(pos: IVec2) -> Option<usize> {
        if pos.x >= 0 && pos.x < CHUNK_SIZE.x && pos.y >= 0 && pos.y < CHUNK_SIZE.y {
            Some((pos.y * CHUNK_SIZE.x + pos.x) as usize)
        } else {
            None
        }
    }

    pub fn global_pos_to_chunk_pos(global_pos: IVec2) -> IVec2 {
        // Adjust the global_pos for modulo operation to ensure positive results
        let mod_x = ((global_pos.x % CHUNK_SIZE.x) + CHUNK_SIZE.x) % CHUNK_SIZE.x;
        let mod_y = ((global_pos.y % CHUNK_SIZE.y) + CHUNK_SIZE.y) % CHUNK_SIZE.y;
        IVec2::new(mod_x, mod_y)
    }

    pub fn check_bounds(pos: IVec2) -> bool {
        return Chunk::ivec_to_vec_index(pos).is_some();
    }
}

#[derive(Resource)]
pub struct CellWorld {
    pub chunks: HashMap<IVec2, Chunk>,
    pub chunk_count: i32,
}

impl Default for CellWorld {
    fn default() -> Self {
        Self {
            chunks: Default::default(),
            chunk_count: 0,
        }
    }
}

impl CellWorld {
    pub fn insert(&mut self, pos: IVec2, entity: Option<Entity>) {
        match self.get_mut_chunk(pos) {
            Some(chunk) => chunk.insert(Chunk::global_pos_to_chunk_pos(pos), entity),
            None => {
                let mut new_chunk = Chunk::default();
                new_chunk.insert(Chunk::global_pos_to_chunk_pos(pos), entity);
                self.chunks
                    .insert(CellWorld::calculate_chunk_pos(pos), new_chunk);
                self.chunk_count += 1;
            }
        }
    }

    pub fn get_mut_chunk(&mut self, pos: IVec2) -> Option<&mut Chunk> {
        self.chunks.get_mut(&CellWorld::calculate_chunk_pos(pos))
    }

    pub fn get_chunk(&self, pos: IVec2) -> Option<&Chunk> {
        self.chunks.get(&CellWorld::calculate_chunk_pos(pos))
    }

    pub fn is_cell_empty(&self, pos: IVec2) -> bool {
        match self.get(pos) {
            None => true,
            Some(_) => false,
        }
    }

    pub fn get(&self, pos: IVec2) -> Option<Entity> {
        let chunk_pos = CellWorld::calculate_chunk_pos(pos);
        self.chunks
            .get(&chunk_pos)
            .and_then(|chunk| chunk.get(Chunk::global_pos_to_chunk_pos(pos)))
    }

    pub fn get_mut(&mut self, pos: IVec2) -> Option<&mut Option<Entity>> {
        let chunk_pos = CellWorld::calculate_chunk_pos(pos);
        self.chunks.get_mut(&chunk_pos)?.get_mut(pos % CHUNK_SIZE)
    }

    fn calculate_chunk_pos(pos: IVec2) -> IVec2 {
        // Adjust the position before division to handle negative coordinates correctly
        let div_x = if pos.x < 0 {
            (pos.x + 1 - CHUNK_SIZE.x) / CHUNK_SIZE.x
        } else {
            pos.x / CHUNK_SIZE.x
        };
        let div_y = if pos.y < 0 {
            (pos.y + 1 - CHUNK_SIZE.y) / CHUNK_SIZE.y
        } else {
            pos.y / CHUNK_SIZE.y
        };
        IVec2::new(div_x, div_y)
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::entity::Entity;
    use bevy::math::IVec2;

    use crate::resources::cell_world::CellWorld;

    #[test]
    fn if_cell_world_is_empty_fn() {
        let mut cell_world = CellWorld::default();
        cell_world.insert(IVec2::new(10, 10), Some(Entity::from_raw(10)));
        cell_world.insert(IVec2::new(-10, 10), Some(Entity::from_raw(10)));
        println!("Cell at (1, 1): {:?}", cell_world.get(IVec2::new(1, 1)));
        assert_eq!(true, cell_world.is_cell_empty(IVec2::new(1, 1)));
        assert_eq!(false, cell_world.is_cell_empty(IVec2::new(10, 10)));
        assert_eq!(false, cell_world.is_cell_empty(IVec2::new(-10, 10)));
    }
}
