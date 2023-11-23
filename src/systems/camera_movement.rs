use crate::components::{Cell, MainCamera};
use bevy::prelude::*;

pub fn move_camera(
    mut camera_q: Query<&mut Transform, With<MainCamera>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut camera_transform = camera_q.single_mut();
    let mut move_dir = Vec2::new(0f32, 0f32);
    let speed = 250;
    if keys.pressed(KeyCode::W) {
        move_dir.y = 1f32;
        println!("moving up")
    }
    if keys.pressed(KeyCode::S) {
        move_dir.y -= 1f32;
        println!("moving down")
    }
    if keys.pressed(KeyCode::A) {
        move_dir.x -= 1f32;
        println!("moving left")
    }
    if keys.pressed(KeyCode::D) {
        move_dir.x += 1f32;
        println!("moving right")
    }

    camera_transform.translation += (move_dir * speed as f32 * time.delta_seconds()).extend(0f32);
}
