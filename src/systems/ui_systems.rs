use crate::components::{Cell, MainCamera};
use crate::resources::cell_world::CellWorld;
use crate::resources::{CellAssets, CellTypeToSpawn, CursorPosition, EguiHoverState, Selected};
use crate::utils::{align_to_grid, position_to_cell_coords};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn spawn_cell_type(
    mut contexts: EguiContexts,
    cell_type_to_spawn: ResMut<CellTypeToSpawn>,
    cell_assets: Res<CellAssets>,
) {
    let _show = egui::Window::new("Cell Type").show(contexts.ctx_mut(), |ui| {
        match cell_type_to_spawn.selected.clone() {
            Some(mut selected) => {
                egui::ComboBox::from_label("Select one!")
                    .selected_text(format!("{}", selected.name))
                    .show_ui(ui, |ui| {
                        for cell_asset in cell_assets.handles.iter() {
                            if ui
                                .selectable_value(
                                    &mut selected,
                                    Selected {
                                        name: cell_asset.0.to_string(),
                                        handle: cell_asset.1.clone(),
                                    },
                                    format!("{:?}", cell_asset.0),
                                )
                                .clicked()
                            {
                                info!("Selected: {}", cell_asset.0);
                            }
                        }
                    });
            }
            None => {
                return;
            }
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

pub fn show_cell_count(mut contexts: EguiContexts, cell_count: ResMut<CellWorld>) {
    egui::Window::new("Cell count").show(contexts.ctx_mut(), |ui| {
        ui.label(cell_count.cell_count.to_string())
    });
}

pub fn check_is_empty_on_mouse_pos(
    cursor_positions: Res<CursorPosition>,
    world: Res<CellWorld>,
    mut contexts: EguiContexts,
    cells_query: Query<&Cell>,
) {
    let grid_pos = position_to_cell_coords(cursor_positions.pos);
    let value = world.get(grid_pos.0, grid_pos.1);
    let window = egui::Window::new("Is empty on mouse position:");
    match value {
        Some(e) => {
            if let Ok(cell) = cells_query.get(e) {
                window.show(contexts.ctx_mut(), |ui| {
                    ui.label(format!("{}", cell.cell_type))
                });
            }
        }
        None => {
            window.show(contexts.ctx_mut(), |ui| ui.label("empty"));
        }
    }
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
