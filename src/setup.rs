// setup.rs
use crate::{
    components::MainCamera,
    enums::CellPhysicsType,
    events::{RemoveCellEvent, SpawnCellEvent},
    resources::{
        cell_world::CellWorld, CellMesh, CellTypeToSpawn, CursorPosition, EguiHoverState,
        SimulateWorldState, CellAssets,
    },
    systems::{
        camera::{move_camera, zoom_camera},
        cell_management::{
            remove_cell, spawn_cell, spawn_cell_on_touch, spawn_or_remove_cell_on_click,
        },
        physics::{blood_stone_physics, fluid_physics, sand_physics},
        ui_systems::{
            check_egui_hover, check_is_empty_on_mouse_pos, my_cursor_system, show_cell_count,
            spawn_cell_type,
        },
        window_management::set_window_icon,
    }, assets::CellAsset,
};
use bevy::{prelude::*, window::PresentMode, utils::HashMap};

use bevy_egui::EguiPlugin;
use bevy_enum_filter::prelude::AddEnumFilter;
use bevy_fps_counter::FpsCounterPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_window_icon)
            .add_systems(Startup, setup)
            .insert_resource(ClearColor(Color::rgb(0.0, 0.170, 0.253)))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "rusty sand".into(),
                    resolution: (1280., 720.).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }))
            .add_plugins(EguiPlugin)
            .add_enum_filter::<CellPhysicsType>()
            .add_systems(Update, spawn_cell_type)
            .add_systems(Update, my_cursor_system)
            .add_systems(Update, spawn_or_remove_cell_on_click)
            .add_plugins(WorldInspectorPlugin::new())
            .add_systems(Update, (sand_physics, fluid_physics, blood_stone_physics))
            .add_systems(Update, move_camera)
            .insert_resource(CellWorld::default())
            .insert_resource(CellTypeToSpawn::default())
            .insert_resource(EguiHoverState::default())
            .insert_resource(CursorPosition::default())
            .insert_resource(SimulateWorldState::default())
            .register_type::<SimulateWorldState>()
            .add_plugins(FpsCounterPlugin)
            .add_systems(Update, spawn_cell_on_touch)
            .add_systems(Update, zoom_camera)
            .add_systems(Update, show_cell_count)
            .add_systems(Update, process_loaded_assets)
            .init_asset::<CellAsset>()
            .add_systems(
                FixedUpdate,
                (
                    spawn_cell,
                    remove_cell,
                    check_is_empty_on_mouse_pos,
                    //cell_list_ui,
                    check_egui_hover,
                ),
            )
            .add_event::<SpawnCellEvent>()
            .add_event::<RemoveCellEvent>();
    }
}

fn setup(mut commands: Commands, meshes: ResMut<Assets<Mesh>>, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.insert_resource(CellMesh::from_world(meshes));
    
    let paths = vec!["cells/blood_stone.ron", "cells/blood.ron", "cells/sand.ron", "cells/stone.ron", "cells/water.ron"];

    for path in paths {
        let asset_handle = asset_server.load::<CellAsset>(path);
        commands.spawn_empty().insert(asset_handle);
    }
}


fn process_loaded_assets(
    mut commands: Commands,
    query: Query<(Entity, &Handle<CellAsset>)>,
    mut cell_assets: ResMut<CellAssets>,
    cell_assets_storage: Res<Assets<CellAsset>>,
) {
    for (entity, handle) in query.iter() {
        if let Some(cell_asset) = cell_assets_storage.get(handle) {
            // Now you have access to cell_asset data like cell_type_name
            cell_assets.handles.insert(cell_asset.cell_type_name.clone(), handle.clone());

            // Remove the entity to avoid reprocessing
            commands.entity(entity).despawn();
        }
    }
}
