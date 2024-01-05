use crate::components::{Cell, MainCamera};
use crate::enums::{CellPhysicsType, CellType, CELL_SIZE};
use crate::events::SpawnCellEvent;
use crate::resources::cell_world::CellWorld;
use crate::resources::{CellMesh, CellTypeToSpawn, CursorPosition, EguiHoverState, SandMaterials};
use crate::utils::{align_to_grid, position_to_cell_coords};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub fn spawn_cell_on_click(
    buttons: Res<Input<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    mut ev_spawn_cell: EventWriter<SpawnCellEvent>,
    state: ResMut<EguiHoverState>,
    cell_type_to_spawn: Res<CellTypeToSpawn>,
) {
    if buttons.pressed(MouseButton::Left) && !state.is_hovered {
        ev_spawn_cell.send(SpawnCellEvent {
            pos: cursor_position.pos,
            cell_type: cell_type_to_spawn.type_to_select,
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
            let mut new_touch_position = camera
                .viewport_to_world_2d(camera_transform, touch_position)
                .unwrap();
            new_touch_position = align_to_grid(new_touch_position);

            ev_spawn_cell.send(SpawnCellEvent {
                pos: new_touch_position,
                cell_type: cell_type_to_spawn.type_to_select,
            });
            return;
        }
    }
}

pub fn spawn_cell(
    mut commands: Commands,
    materials: Res<SandMaterials>,
    cell_mesh: Res<CellMesh>,
    mut cell_world: ResMut<CellWorld>,
    mut ev_spawn_cell: EventReader<SpawnCellEvent>,
) {
    for ev in ev_spawn_cell.read() {
        let grid_pos = position_to_cell_coords(ev.pos);
        if cell_world.is_cell_empty(grid_pos) {
            if let Some(&material_index) = materials.color_ids.get(&ev.cell_type) {
                // Access the material using the material index
                if let Some(material) = materials.materials.get(material_index) {
                    cell_world.insert(
                        grid_pos.0,
                        grid_pos.1,
                        Some(
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
                                        cell_type: ev.cell_type,
                                    },
                                    get_physics_component(ev.cell_type),
                                ))
                                .id(),
                        ),
                    );
                } else {
                    warn!("Material for cell type {:?} not found", ev.cell_type);
                }
            } else {
                warn!("No material index for cell type {:?}", ev.cell_type);
            }
        }
    }
}

pub fn get_physics_component(cell: CellType) -> CellPhysicsType {
    match cell {
        CellType::Sand => CellPhysicsType::Sand,
        CellType::Stone => CellPhysicsType::Static,
        CellType::BloodStone => CellPhysicsType::BloodStone,
        CellType::Blood => CellPhysicsType::Fluid,
    }
}
