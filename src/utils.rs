// utils.rs
// Implement utility functions like screen_to_world, etc.

use bevy::prelude::*;

use crate::components::MainCamera;

pub fn get_screen_center(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Vec2 {
    let window = windows.single();
    let mut screen_center = Vec2::new(window.width() / 2.0, window.height() as f32 / 2.0);
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, screen_center) {
        screen_center = world_position;
        screen_center.x = (screen_center.x as i32) as f32;
        screen_center.y = (screen_center.y as i32) as f32;
        return screen_center;
    }

    return screen_center;
}
