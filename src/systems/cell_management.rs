use crate::components::{Cell, CursorPosition, MainCamera};
use crate::enums::{CellType, CELL_SIZE};
use crate::events::SpawnCellEvent;
use crate::resources::{CellMesh, CellTypeToSpawn, CellWorld, SandMaterials};
use crate::utils::round_pos_to_grid;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub fn spawn_cell_on_click(
    buttons: Res<Input<MouseButton>>,
    cursor_positions: Query<&mut CursorPosition>,
    mut ev_spawn_cell: EventWriter<SpawnCellEvent>,
) {
    if buttons.pressed(MouseButton::Left) {
        let mut new_cursor_position = cursor_positions.single().pos;
        new_cursor_position = round_pos_to_grid(new_cursor_position);
        ev_spawn_cell.send(SpawnCellEvent {
            pos: new_cursor_position,
        });
    }
}

pub fn spawn_cell_on_touch(
    touches: Res<Touches>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut ev_spawn_cell: EventWriter<SpawnCellEvent>,
) {
    let (camera, camera_transform) = camera_q.single();

    for finger in touches.iter() {
        if touches.just_pressed(finger.id()) {
            let touch_position = finger.position();
            let mut new_touch_position = camera
                .viewport_to_world_2d(camera_transform, touch_position)
                .unwrap();
           new_touch_position = round_pos_to_grid(new_touch_position);

            ev_spawn_cell.send(SpawnCellEvent {
                pos: new_touch_position,
            });
            return;
        }
    }
}

pub fn spawn_cell(
    mut commands: Commands,
    materials: Res<SandMaterials>,
    cell_mesh: Res<CellMesh>,
    cell_type_to_spawn: ResMut<CellTypeToSpawn>,
    mut cell_world: ResMut<CellWorld>,
    mut ev_spawn_cell: EventReader<SpawnCellEvent>,
) {
    for ev in ev_spawn_cell.read() {
        if let Some(&material_index) = materials.color_ids.get(&cell_type_to_spawn.type_to_select) {
            // Access the material using the material index
            if let Some(material) = materials.materials.get(material_index) {
                cell_world.insert_by_pos_if_empty(
                    ev.pos,
                    commands
                        .spawn((
                            MaterialMesh2dBundle {
                                mesh: cell_mesh.mesh.clone(),
                                transform: Transform {
                                    translation: ev.pos.extend(0.0),
                                    scale: CELL_SIZE,
                                    ..Default::default()
                                },
                                material: material.clone(),
                                ..Default::default()
                            },
                            Cell {
                                cell_type: cell_type_to_spawn.type_to_select,
                            },
                        ))
                        .id(),
                );
            } else {
                // Handle the case where the material is not found
                warn!(
                    "Material for cell type {:?} not found",
                    cell_type_to_spawn.type_to_select
                );
            }
        } else {
            // Handle the case where the material index is not found
            warn!(
                "No material index for cell type {:?}",
                cell_type_to_spawn.type_to_select
            );
        }
    }
    // Get the material index for the given cell type
}

pub fn physics(
    mut cells_query: Query<(Entity, &mut Cell, &mut Transform)>,
    mut cell_world: ResMut<CellWorld>,
) {
    for (entity, cell, mut transform) in cells_query.iter_mut() {
        match cell.cell_type {
            CellType::Sand => {
                // Calculate the grid position below the current cell
                let below_x = (transform.translation.x / CELL_SIZE.x).floor() as usize;
                let below_y = ((transform.translation.y - CELL_SIZE.y) / CELL_SIZE.y).floor() as usize;

                // Check if the position below is empty
                if cell_world.get(below_x, below_y).is_none() {
                    // Move the cell down if empty
                    transform.translation.y -= CELL_SIZE.y;

                    // Update the CellWorld grid
                    cell_world.insert(below_x, below_y, entity);
                    cell_world.insert(below_x, below_y + 1, Entity::from_raw(0));
                    // Assuming 0 is used for empty/invalid entities
                }
            }
            _ => {}
        }
    }
}
