// utils.rs
// Implement utility functions like screen_to_world, etc.

use bevy::prelude::*;

use crate::{components::MainCamera, enums::CELL_SIZE};

pub fn get_screen_center(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Vec2 {
    let window = windows.single();
    let mut screen_center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, screen_center) {
        screen_center = align_to_grid(world_position);
        return screen_center;
    }

    screen_center
}

pub fn align_to_grid(mut pos: Vec2) -> Vec2 {
    pos.x = (pos.x / CELL_SIZE.x).floor() * CELL_SIZE.x;
    pos.y = (pos.y / CELL_SIZE.y).floor() * CELL_SIZE.y;

    pos
}

pub trait XY {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

impl XY for Vec2 {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

impl XY for Vec3 {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

pub fn position_to_cell_coords<T: XY>(pos: T) -> (isize, isize) {
    (
        (pos.x() / CELL_SIZE.x).floor() as isize,
        (pos.y() / CELL_SIZE.y).floor() as isize,
    )
}
