// utils.rs
// Implement utility functions like screen_to_world, etc.

use bevy::prelude::*;

use crate::components::MainCamera;

pub fn screen_to_world(
    touch_position: Vec2,
    windows_query: Query<&Window>,
    cameras: Query<(&Transform, &Camera), With<MainCamera>>,
) -> Vec3 {
    let window = windows_query.iter().next().unwrap();

    // For the purpose of this example, we assume there's one main camera.
    // Adjust as necessary for your setup.
    let (camera_transform, camera) = cameras.iter().next().unwrap();

    // Screen to NDC
    let ndc = Vec3::new(
        (touch_position.x / window.width() as f32) * 2.0 - 1.0,
        (touch_position.y / window.height() as f32) * 2.0 - 1.0,
        0.5, // Middle of the near/far plane
    );

    // NDC to world space
    let world_position = camera.projection_matrix().inverse() * Vec4::new(ndc.x, ndc.y, ndc.z, 1.0);

    (camera_transform.compute_matrix() * world_position).truncate()
}
