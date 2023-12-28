// setup.rs
use crate::{
    components::{CursorPosition, MainCamera},
    events::SpawnCellEvent,
    resources::{CellMesh, CellTypeToSpawn, CellWorld, HandleInputOnMouse, SandMaterials},
    systems::{
        camera::{move_camera, zoom_camera},
        cell_management::{physics, spawn_cell, spawn_cell_on_click, spawn_cell_on_touch},
        ui_systems::{my_cursor_system, show_cell_count, spawn_cell_type},
        window_management::set_window_icon, input_handling::change_state_on_handle_input_on_mouse,
    },
};
use bevy::{prelude::*, window::PresentMode};

use bevy_egui::EguiPlugin;
use bevy_fps_counter::FpsCounterPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_window_icon)
            .add_systems(Startup, setup)
            .insert_resource(ClearColor(Color::rgb(0.0, 0.170, 0.253)))
            .insert_resource(HandleInputOnMouse::default())
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "rusty sand".into(),
                    resolution: (500., 300.).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }))
            .add_plugins(EguiPlugin)
            .add_systems(Update, spawn_cell_type)
            .add_systems(Update, my_cursor_system)
            .add_systems(Update, spawn_cell_on_click)
            .add_plugins(WorldInspectorPlugin::new())
            .add_systems(Update, physics)
            .add_systems(Update, move_camera)
            .insert_resource(CellWorld::default())
            .insert_resource(CellTypeToSpawn::default())
            .insert_resource(HandleInputOnMouse::default())
            .add_plugins(FpsCounterPlugin)
            .add_systems(Update, spawn_cell_on_touch)
            .add_systems(Update, zoom_camera)
            .add_systems(Update, show_cell_count)
            .add_systems(Update, (spawn_cell, change_state_on_handle_input_on_mouse))
            .add_event::<SpawnCellEvent>();
    }
}

fn setup(
    mut commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(CursorPosition {
        pos: Vec2 { x: 0f32, y: 0f32 },
    });
    commands.spawn((Camera2dBundle::default(), MainCamera));

    commands.insert_resource(SandMaterials::from_world(materials));
    commands.insert_resource(CellMesh::from_world(meshes));
}
