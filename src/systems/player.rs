use bevy::prelude::*;

use crate::{
    components::Player,
    resources::cell_world::CellWorld,
    utils::{align_to_grid, position_to_cell_coords},
};

pub fn move_player(
    mut player_q: Query<&mut Transform, With<Player>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    world: ResMut<CellWorld>,
    mut gizmos: Gizmos,
) {
    let mut player_transform = player_q.single_mut();
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

    let transform_to_try_to_move = player_transform.translation
        + (move_dir * speed as f32 * time.delta_seconds()).extend(0f32);

    gizmos.rect_2d(
        player_transform.translation.xy(),
        0.0,
        Vec2 { x: 1.0, y: 1.0 },
        Color::GOLD,
    );

    if world
        .get(position_to_cell_coords(align_to_grid(
            transform_to_try_to_move.xy(),
        )))
        .is_none()
    {
        player_transform.translation = transform_to_try_to_move;
        gizmos.rect_2d(
            align_to_grid(transform_to_try_to_move.xy()),
            0.0,
            Vec2 { x: 1.0, y: 1.0 },
            Color::GREEN,
        );
    } else {
        gizmos.rect_2d(
            align_to_grid(transform_to_try_to_move.xy()),
            0.0,
            Vec2 { x: 1.0, y: 1.0 },
            Color::RED,
        );
    }
}
