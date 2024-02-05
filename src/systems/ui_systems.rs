use crate::components::{Cell, MainCamera};
use crate::resources::cell_world::CellWorld;
use crate::resources::{CellAssets, CellTypeToSpawn, CursorPosition, EguiHoverState, Selected};
use crate::utils::{align_to_grid, position_to_cell_coords};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn spawn_cell_type(
    mut contexts: EguiContexts,
    mut cell_type_to_spawn: ResMut<CellTypeToSpawn>, // Ensure this is mutable
    cell_assets: Res<CellAssets>,
) {
    let _show = egui::Window::new("Cell Type").show(contexts.ctx_mut(), |ui| {
        if let Some(mut selected) = cell_type_to_spawn.selected.clone() {
            // Make sure selected is mutable
            egui::ComboBox::from_label("Select one!")
                .selected_text(format!("{}", selected.name))
                .show_ui(ui, |ui| {
                    for cell_asset in cell_assets.handles.iter() {
                        if ui
                            .selectable_value(
                                &mut selected, // Mutable reference
                                Selected {
                                    name: cell_asset.0.to_string(),
                                    handle: cell_asset.1.clone(),
                                },
                                format!("{:?}", cell_asset.0),
                            )
                            .clicked()
                        {
                            cell_type_to_spawn.selected = Some(selected.clone()); // Update the state
                            info!("Selected: {}", cell_asset.0);
                        }
                    }
                });
        }
    });
}

pub fn my_cursor_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut contexts: EguiContexts,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let window = windows.get_single();
    let (camera, camera_transform) = camera_q.single();

    match window {
        Ok(window) => {
            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
            {
                cursor_position.pos = align_to_grid(world_position);
            }

            egui::Window::new("Cursor Position aligned to grid").show(contexts.ctx_mut(), |ui| {
                ui.label(cursor_position.pos.to_string());
            });
        }
        Err(_) => return,
    }
}

pub fn show_cell_count(mut contexts: EguiContexts, world: ResMut<CellWorld>) {
    let mut cell_count = 0;
    for chunk in &world.chunks {
        cell_count += chunk.1.cell_count;
    }
    egui::Window::new("Cell count").show(contexts.ctx_mut(), |ui| {
        ui.label(cell_count.to_string());
        ui.label(world.chunk_count.to_string());
    });
}

pub fn check_is_empty_on_mouse_pos(
    cursor_positions: Res<CursorPosition>,
    world: Res<CellWorld>,
    mut contexts: EguiContexts,
    cells_query: Query<&Cell>,
) {
    let grid_pos = position_to_cell_coords(cursor_positions.pos);
    let value = world.get(grid_pos);
    let is_empty_text: String = match value {
        Some(e) => {
            if let Ok(cell) = cells_query.get(e) {
                cell.cell_type.clone()
            } else {
                "empty".to_string()
            }
        }
        None => "empty".to_string(),
    };
    egui::Window::new("Is empty on mouse position:").show(contexts.ctx_mut(), |ui| {
        ui.label(is_empty_text);
    });
}

pub fn cell_list_ui(query: Query<(&Cell, &Transform)>, mut contexts: EguiContexts) {
    egui::Window::new("Cells list:").show(contexts.ctx_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (cell, transform) in query.iter() {
                ui.label(format!("{}, {:?}", cell.cell_type, transform.translation));
            }
        });
    });
}

pub fn check_egui_hover(mut contexts: EguiContexts, mut state: ResMut<EguiHoverState>) {
    state.is_hovered = contexts.ctx_mut().is_pointer_over_area();
}
