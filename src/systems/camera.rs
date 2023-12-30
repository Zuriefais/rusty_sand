use crate::{
    components::MainCamera,
    resources::{CursorPosition, EguiHoverState},
};
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

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
    }
    if keys.pressed(KeyCode::S) {
        move_dir.y -= 1f32;
    }
    if keys.pressed(KeyCode::A) {
        move_dir.x -= 1f32;
    }
    if keys.pressed(KeyCode::D) {
        move_dir.x += 1f32;
    }

    camera_transform.translation += (move_dir * speed as f32 * time.delta_seconds()).extend(0f32);
}

pub fn zoom_camera(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection)>,
    cursor_position: Res<CursorPosition>,
    state: ResMut<EguiHoverState>,
) {
    if state.is_hovered {
        return;
    }
    for event in mouse_wheel_events.read() {
        let zoom_amount = match event.unit {
            MouseScrollUnit::Line => 0.1 * event.y, // Adjust the multiplier as needed
            MouseScrollUnit::Pixel => 0.01 * event.y, // Adjust the multiplier as needed
        };

        for (mut transform, mut projection) in query.iter_mut() {
            // 1. Calculate the mouse position in world coordinates
            let cursor_position = cursor_position.pos.extend(0f32);

            // 2. Calculate the direction vector from camera to mouse position
            let direction = (cursor_position - transform.translation).normalize();
            let zoom_factor = 1.0 + zoom_amount; // This factor increases or decreases depending on the zoom_amount

            // 3. Move the camera towards/away from the mouse position
            transform.translation += direction * zoom_factor; // Adjust 'zoom_factor' based on your needs

            // Adjusting the scale for zoom effect
            projection.scale -= zoom_amount;
            projection.scale = projection.scale.clamp(0.1, 10.0); // Ensuring scale is within reasonable range
        }
    }
}
