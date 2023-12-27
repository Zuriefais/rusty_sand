use crate::components::{CursorPosition, MainCamera};
use crate::enums::CellType;
use crate::resources::{CellTypeToSpawn, CellWorld};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn spawn_cell_type(
    mut contexts: EguiContexts,
    mut cell_type_to_spawn: ResMut<CellTypeToSpawn>,
) {
    egui::Window::new("cell type").show(contexts.ctx_mut(), |ui| {
        egui::ComboBox::from_label("Select one!")
            .selected_text(format!("{:?}", cell_type_to_spawn.type_to_select))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut &cell_type_to_spawn.type_to_select,
                    &CellType::Sand,
                    "Sand",
                );
                ui.selectable_value(
                    &mut &cell_type_to_spawn.type_to_select,
                    &CellType::Stone,
                    "Stone",
                );
            });
    });
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

pub fn show_cell_count(mut contexts: EguiContexts, cell_count: ResMut<CellWorld>) {
    egui::Window::new("Cell count").show(contexts.ctx_mut(), |ui| {
        ui.label(cell_count.cell_count.to_string())
    });
}
