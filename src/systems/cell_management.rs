use crate::components::MainCamera;
use crate::events::{RemoveCellEvent, SpawnCellEvent};
use crate::resources::cell_world::CellWorld;
use crate::resources::{CellTypeToSpawn, CursorPosition, EguiHoverState};
use crate::utils::{align_to_grid, position_to_cell_coords};
use bevy::prelude::*;

pub fn spawn_or_remove_cell_on_click(
    buttons: Res<Input<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    mut ev_spawn_cell: EventWriter<SpawnCellEvent>,
    mut ev_remove_cell: EventWriter<RemoveCellEvent>,
    state: ResMut<EguiHoverState>,
    cell_type_to_spawn: Res<CellTypeToSpawn>,
) {
    if buttons.pressed(MouseButton::Left) && !state.is_hovered {
        match &cell_type_to_spawn.selected {
            Some(selected) => {
                ev_spawn_cell.send(SpawnCellEvent {
                    pos: cursor_position.pos,
                    cell_type: selected.handle.clone(),
                });
            }
            None => {}
        }
    } else if buttons.pressed(MouseButton::Right) && !state.is_hovered {
        ev_remove_cell.send(RemoveCellEvent {
            pos: position_to_cell_coords(cursor_position.pos),
        });
    }
}

pub fn spawn_cell_on_touch(
    touches: Res<Touches>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut ev_spawn_cell: EventWriter<SpawnCellEvent>,
    cell_type_to_spawn: Res<CellTypeToSpawn>,
) {
    let (camera, camera_transform) = camera_q.single();

    for finger in touches.iter() {
        if touches.just_pressed(finger.id()) {
            let touch_position = finger.position();
            let mut touch_position = camera
                .viewport_to_world_2d(camera_transform, touch_position)
                .unwrap();
            touch_position = align_to_grid(touch_position);

            match &cell_type_to_spawn.selected {
                Some(selected) => {
                    ev_spawn_cell.send(SpawnCellEvent {
                        pos: touch_position,
                        cell_type: selected.handle.clone(),
                    });
                }
                None => {
                    return;
                }
            }
            return;
        }
    }
}

pub fn spawn_cell(
    mut cell_world: ResMut<CellWorld>,
    mut ev_spawn_cell: EventReader<SpawnCellEvent>,
) {
    for ev in ev_spawn_cell.read() {
        let grid_pos = position_to_cell_coords(ev.pos);

        if cell_world.is_cell_empty(grid_pos) {
            cell_world.insert(grid_pos, Some((ev.cell_type, Vec2::ZERO)));
        }
    }
}

pub fn remove_cell(mut ev_remove_cell: EventReader<RemoveCellEvent>, mut world: ResMut<CellWorld>) {
    for event in ev_remove_cell.read() {
        world.insert(event.pos, None);
    }
}
