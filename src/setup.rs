// setup.rs
use crate::{
    assets::{CellAsset, CellAssetLoader, ConfigAsset, ConfigAssetLoader},
    components::{MainCamera, Cell},
    enums::CellPhysicsType,
    events::{RemoveCellEvent, SpawnCellEvent},
    resources::{
        cell_world::CellWorld, CellAssets, CellMesh, CellTypeToSpawn, CursorPosition,
        EguiHoverState, SimulateWorldState, Selected,
    },
    systems::{
        camera::{move_camera, zoom_camera},
        cell_management::{
            remove_cell, spawn_cell, spawn_cell_on_touch, spawn_or_remove_cell_on_click,
        },
        physics::{blood_stone_physics, fluid_physics, sand_physics},
        ui_systems::{
            cell_list_ui, check_egui_hover, check_is_empty_on_mouse_pos, my_cursor_system,
            show_cell_count, spawn_cell_type,
        },
        window_management::set_window_icon,
    },
};
use bevy::{prelude::*, utils::HashMap, window::PresentMode};

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
            .register_type::<Cell>()
            .add_plugins(FpsCounterPlugin)
            .add_systems(Update, spawn_cell_on_touch)
            .add_systems(Update, zoom_camera)
            .add_systems(Update, show_cell_count)
            .add_systems(Update, process_loaded_assets)
            .add_systems(Update, load_cell_assets)
            .init_asset::<CellAsset>()
            .init_asset::<ConfigAsset>()
            .register_asset_loader(CellAssetLoader)
            .register_asset_loader(ConfigAssetLoader)
            .add_systems(
                FixedUpdate,
                (
                    spawn_cell,
                    remove_cell,
                    cell_list_ui,
                    check_egui_hover,
                    check_is_empty_on_mouse_pos,
                ),
            )
            .add_event::<SpawnCellEvent>()
            .add_event::<RemoveCellEvent>();
    }
}

fn setup(mut commands: Commands, meshes: ResMut<Assets<Mesh>>, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands.insert_resource(CellMesh::from_world(meshes));

    commands.insert_resource(CellAssets {
        handles: HashMap::new(),
    });

    info!("loading config asset.....");
    let config_handle = asset_server.load::<ConfigAsset>("config.config");
    commands.spawn_empty().insert(config_handle);
}

fn load_cell_assets(
    config_handle_entities: Query<(Entity, &Handle<ConfigAsset>)>,
    config_assets: Res<Assets<ConfigAsset>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for entity in config_handle_entities.iter() {
        if let Some(config) = config_assets.get(entity.1) {
            for path in &config.cell_paths {
                info!("loading cell at path: {}", path);
                let asset_handle = asset_server.load::<CellAsset>(path);
                commands.spawn_empty().insert(asset_handle);
            }
        }
        commands.entity(entity.0).despawn();
    }  
}

fn process_loaded_assets(
    mut commands: Commands,
    query: Query<(Entity, &Handle<CellAsset>)>,
    mut cell_assets: ResMut<CellAssets>,
    cell_assets_storage: Res<Assets<CellAsset>>,
    mut cell_type: ResMut<CellTypeToSpawn>
) {
    for (entity, handle) in query.iter() {
        if let Some(cell_asset) = cell_assets_storage.get(handle) {
            // Now you have access to cell_asset data like cell_type_name
            cell_assets
                .handles
                .insert(cell_asset.cell_type_name.clone(), handle.clone());
            info!("{:?}", cell_asset);
            // Remove the entity to avoid reprocessing
            cell_type.selected = Some(Selected { name: cell_asset.cell_type_name.clone(), handle: handle.clone() });
            commands.entity(entity).despawn();
        }
    }
}
