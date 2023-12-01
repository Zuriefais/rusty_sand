use crate::components::{Cell, CellTypeToSpawn, CursorPosition, MainCamera};
use crate::enums::{CellType, CELL_SIZE};
use crate::resources::{CellMesh, CellWorld, SandMaterials};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub fn spawn_cell_on_click(
    commands: Commands,
    materials: Res<SandMaterials>,
    buttons: Res<Input<MouseButton>>,
    cursor_positions: Query<&mut CursorPosition>,
    query: Query<&CellTypeToSpawn>,
    cell_mesh: Res<CellMesh>,
    cell_world: ResMut<CellWorld>,
) {
    if buttons.pressed(MouseButton::Left) {
        let mut new_cursor_position = cursor_positions.single().pos;
        new_cursor_position.x -= (new_cursor_position.x as i32 % CELL_SIZE.x as i32) as f32;
        new_cursor_position.y -= (new_cursor_position.y as i32 % CELL_SIZE.x as i32) as f32;
        spawn_cell(
            commands,
            materials,
            cell_mesh,
            new_cursor_position,
            query.single().type_to_select,
            cell_world,
        );
    }
}

pub fn spawn_cell_on_touch(
    commands: Commands,
    materials: Res<SandMaterials>,
    query: Query<&CellTypeToSpawn>,
    touches: Res<Touches>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    cell_mesh: Res<CellMesh>,
    cell_world: ResMut<CellWorld>,
) {
    let (camera, camera_transform) = camera_q.single();

    for finger in touches.iter() {
        if touches.just_pressed(finger.id()) {
            let touch_position = finger.position();
            let mut new_touch_position = camera
                .viewport_to_world_2d(camera_transform, touch_position)
                .unwrap();
            new_touch_position.x -= (new_touch_position.x as i32 % CELL_SIZE.x as i32) as f32;
            new_touch_position.y -= -(new_touch_position.y as i32 % CELL_SIZE.x as i32) as f32;

            spawn_cell(
                commands,
                materials,
                cell_mesh,
                new_touch_position,
                query.single().type_to_select,
                cell_world,
            );
            return;
        }
    }
}

pub fn spawn_test() {
    for number in 1..100 {}
}

pub fn spawn_cell(
    mut commands: Commands,
    materials: Res<SandMaterials>,
    cell_mesh: Res<CellMesh>,
    pos: Vec2,
    cell_type: CellType,
    mut cell_world: ResMut<CellWorld>,
) {
    // Get the material index for the given cell type
    if let Some(&material_index) = materials.color_ids.get(&cell_type) {
        // Access the material using the material index
        if let Some(material) = materials.materials.get(material_index) {
            cell_world.insert_by_pos_if_empty(pos, commands.spawn((
                MaterialMesh2dBundle {
                    mesh: cell_mesh.mesh.clone(),
                    transform: Transform {
                        translation: pos.extend(0.0),
                        scale: CELL_SIZE,
                        ..Default::default()
                    },
                    material: material.clone(),
                    ..Default::default()
                },
                Cell { cell_type },
            )).id());
        } else {
            // Handle the case where the material is not found
            warn!("Material for cell type {:?} not found", cell_type);
        }
    } else {
        // Handle the case where the material index is not found
        warn!("No material index for cell type {:?}", cell_type);
    }
}

pub fn physics(
    mut cells_query: Query<(Entity, &mut Cell, &mut Transform)>,
    mut cell_world: ResMut<CellWorld>
) {
    for (entity, cell, mut transform) in cells_query.iter_mut() {
        match cell.cell_type {
            CellType::Sand => {
                // Calculate the grid position below the current cell
                let below_x = (transform.translation.x / CELL_SIZE.x as f32).floor() as usize;
                let below_y = ((transform.translation.y - CELL_SIZE.y as f32) / CELL_SIZE.y as f32).floor() as usize;

                // Check if the position below is empty
                if cell_world.get(below_x, below_y).is_none() {
                    // Move the cell down if empty
                    transform.translation.y -= CELL_SIZE.y as f32;

                    // Update the CellWorld grid
                    cell_world.insert(below_x, below_y, entity);
                    cell_world.insert(below_x, below_y + 1, Entity::from_raw(0)); // Assuming 0 is used for empty/invalid entities
                }
            }
            _ => {}
        }
    }
}

