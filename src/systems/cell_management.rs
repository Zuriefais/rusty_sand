use crate::components::{Cell, CellTypeToSpawn, CursorPosition, MainCamera};
use crate::enums::{CellType, CELL_SIZE};
use crate::resources::{CellMesh, SandMaterials};
use crate::utils::screen_to_world;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub fn spawn_cell_on_click(
    commands: Commands,
    materials: Res<SandMaterials>,
    buttons: Res<Input<MouseButton>>,
    cursor_positions: Query<&mut CursorPosition>,
    query: Query<&CellTypeToSpawn>,
    cell_pos_query: Query<&Transform, With<Cell>>,
    cell_mesh: Res<CellMesh>,
) {
    if buttons.pressed(MouseButton::Left) {
        let mut new_cursor_position = cursor_positions.single().pos;
        new_cursor_position.x -= (new_cursor_position.x as i32 % CELL_SIZE.x as i32) as f32;
        new_cursor_position.y -= (new_cursor_position.y as i32 % CELL_SIZE.x as i32) as f32;
        for cell_pos in &cell_pos_query {
            if cell_pos.translation == new_cursor_position.extend(0f32) {
                return;
            }
        }

        spawn_cell(
            commands,
            materials,
            cell_mesh,
            new_cursor_position.extend(0f32),
            query.single().type_to_select,
            cell_pos_query,
        );
    }
}

pub fn spawn_cell_on_touch(
    commands: Commands,
    materials: Res<SandMaterials>,
    query: Query<&CellTypeToSpawn>,
    cell_pos_query: Query<&Transform, With<Cell>>,
    touches: Res<Touches>,
    windows: Query<&Window>,
    camera_q: Query<(&Transform, &Camera), With<MainCamera>>,
    cell_mesh: Res<CellMesh>,
) {
    for finger in touches.iter() {
        if touches.just_pressed(finger.id()) {
            let touch_position = finger.position();
            let mut new_touch_position = screen_to_world(touch_position, windows, camera_q);
            new_touch_position.x -= (new_touch_position.x as i32 % CELL_SIZE.x as i32) as f32;
            new_touch_position.y -= -(new_touch_position.y as i32 % CELL_SIZE.x as i32) as f32;

            spawn_cell(
                commands,
                materials,
                cell_mesh,
                new_touch_position,
                query.single().type_to_select,
                cell_pos_query,
            );
            return;
        }
    }
}

pub fn spawn_cell(
    mut commands: Commands,
    materials: Res<SandMaterials>,
    cell_mesh: Res<CellMesh>,
    pos: Vec3,
    cell_type: CellType,
    cell_pos_query: Query<&Transform, With<Cell>>,
) {
    // Check if a cell already exists at the position
    for cell_pos in &cell_pos_query {
        if cell_pos.translation == pos {
            return;
        }
    }

    // Get the material index for the given cell type
    if let Some(&material_index) = materials.color_ids.get(&cell_type) {
        // Access the material using the material index
        if let Some(material) = materials.materials.get(material_index) {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: cell_mesh.mesh.clone(),
                    transform: Transform {
                        translation: pos,
                        scale: CELL_SIZE,
                        ..Default::default()
                    },
                    material: material.clone(),
                    ..Default::default()
                },
                Cell { cell_type },
            ));
        } else {
            // Handle the case where the material is not found
            warn!("Material for cell type {:?} not found", cell_type);
        }
    } else {
        // Handle the case where the material index is not found
        warn!("No material index for cell type {:?}", cell_type);
    }
}

pub fn physics(mut cells_query: Query<(Entity, &mut Cell, &mut Transform)>) {
    let entities: Vec<Entity> = cells_query.iter_mut().map(|(ent, _, _)| ent).collect();

    let mut to_move = Vec::new(); // Vec to track which entities need to be moved

    for i in 0..entities.len() {
        if let Ok((_, cell, transform)) = &cells_query.get(entities[i]) {
            // Note: not getting mutably here
            match cell.cell_type {
                CellType::Sand => {
                    let mut stop = false;

                    for i2 in 0..entities.len() {
                        if let Ok((_, _cell2, transform2)) = cells_query.get(entities[i2]) {
                            // Note: not getting mutably here
                            if transform2.translation
                                == transform.translation
                                    + (Vec3 {
                                        x: 0f32,
                                        y: -10f32,
                                        z: 0f32,
                                    })
                            {
                                stop = true;
                                break;
                            }
                        }
                    }

                    if !stop {
                        to_move.push(entities[i]); // If it should move, save the entity for the next phase
                    }
                }
                _ => {}
            }
        }
    }

    // Phase 2: Mutate the transforms
    for entity in to_move {
        if let Ok((_, _, mut transform)) = cells_query.get_mut(entity) {
            transform.translation.y -= 2f32;
        }
    }
}
