// systems.rs
use crate::components::{Cell, CellTypeToSpawn, CursorPosition, MainCamera};
use crate::enums::{CellType, CELL_SIZE};
use crate::resources::{SandMaterials, CellMesh};
use crate::utils::screen_to_world;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_egui::{egui, EguiContexts};

// Add your systems like set_window_icon, setup, spawn_cell_type, etc.
#[cfg(not(target_arch = "wasm32"))]
pub fn set_window_icon(
    _main_window: Query<Entity, With<bevy::window::PrimaryWindow>>,
    _windows: NonSend<bevy::winit::WinitWindows>,
) {

    //let Some(_primary) = windows.get_window(main_window.single()) else {
    //    return;
    //};

    //let (icon_rgba, icon_width, icon_height) = {
    //let image = image::open("icon.ico")
    //.expect("Failed to open icon path")
    //.into_rgba8();
    //let (width, height) = image.dimensions();
    //let rgba = image.into_raw();
    //(rgba, width, height)
    //};

    //let _icon = winit::window::Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    //primary.set_window_icon(Some(icon));
}

#[cfg(target_arch = "wasm32")]
pub fn set_window_icon() {}

pub fn spawn_cell_type(mut contexts: EguiContexts, mut query: Query<&mut CellTypeToSpawn>) {
    let mut selected = &query.single_mut().type_to_select.clone();
    egui::Window::new("cell type").show(contexts.ctx_mut(), |ui| {
        egui::ComboBox::from_label("Select one!")
            .selected_text(format!("{:?}", selected))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut selected, &CellType::Sand, "Sand");
                ui.selectable_value(&mut selected, &CellType::Stone, "Stone");
            });
    });
    query.single_mut().type_to_select = *selected;
    // Use `ui.enum_select` to create the dropdown menu.
}

pub fn my_cursor_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut contexts: EguiContexts,
    mut cursor_positions: Query<&mut CursorPosition>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    let mut cursor_position = cursor_positions.single_mut();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        cursor_position.pos = world_position;
        cursor_position.pos.x = (cursor_position.pos.x as i32) as f32;
        cursor_position.pos.y = (cursor_position.pos.y as i32) as f32
    }

    egui::Window::new("Cursor Position").show(contexts.ctx_mut(), |ui| {
        ui.label(cursor_position.pos.to_string());
    });
}

pub fn spawn_cell_on_click(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
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
            meshes,
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
    meshes: ResMut<Assets<Mesh>>,
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
                meshes,
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
    _meshes: ResMut<Assets<Mesh>>,
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
                Cell {
                    cell_type,
                },
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
