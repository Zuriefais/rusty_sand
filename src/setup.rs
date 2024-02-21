use std::env;

// setup.rs
use crate::{
    assets::{CellAsset, CellAssetLoader, ConfigAsset, ConfigAssetLoader},
    components::{Cell, MainCamera},
    custom_renderer_plugin::{CustomMaterialPlugin, InstanceData, InstanceMaterialData},
    enums::CellPhysicsType,
    events::{RemoveCellEvent, SpawnCellEvent},
    resources::{
        cell_world::CellWorld, CellAssets, CellMesh, CellTypeToSpawn, CursorPosition,
        EguiHoverState, Selected, SimulateWorldState,
    },
    systems::{
        camera::{move_camera, zoom_camera}, cell_management::{
            remove_cell, spawn_cell, spawn_cell_on_touch, spawn_or_remove_cell_on_click,
        }, physics::{fluid_physics, sand_physics, tap_physics}, render::render, ui_systems::{
            cell_list_ui, check_egui_hover, check_is_empty_on_mouse_pos, chunk_gizmo,
            my_cursor_system, show_cell_count, spawn_cell_type,
        }, window_management::set_window_icon
    },
};
use bevy::{
    prelude::*,
    render::{batching::NoAutomaticBatching, view::NoFrustumCulling},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    utils::HashMap,
    window::PresentMode,
};

use bevy_egui::EguiPlugin;
use bevy_enum_filter::prelude::AddEnumFilter;
use bevy_fps_counter::FpsCounterPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_window_icon)
            .add_systems(Startup, (setup, init_meshes))
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
            .add_plugins((EguiPlugin, CustomMaterialPlugin))
            .add_enum_filter::<CellPhysicsType>()
            .add_systems(Update, spawn_cell_type)
            .add_systems(Update, my_cursor_system)
            .add_systems(Update, spawn_or_remove_cell_on_click)
            .add_systems(Update, (sand_physics, fluid_physics, tap_physics))
            .add_systems(Update, move_camera)
            .insert_resource(CellWorld::default())
            .insert_resource(CellTypeToSpawn::default())
            .insert_resource(EguiHoverState::default())
            .insert_resource(CursorPosition::default())
            .insert_resource(SimulateWorldState::default())
            .register_type::<SimulateWorldState>()
            .register_type::<Cell>()
            .register_type::<CellAssets>()
            .add_plugins(FpsCounterPlugin)
            .add_systems(Update, spawn_cell_on_touch)
            .add_systems(Update, zoom_camera)
            .add_systems(Update, process_loaded_assets)
            .add_systems(Update, load_cell_assets)
            .init_asset::<CellAsset>()
            .init_asset::<ConfigAsset>()
            .register_asset_loader(CellAssetLoader)
            .register_asset_loader(ConfigAssetLoader)
            .add_systems(FixedUpdate, (spawn_cell, remove_cell, check_egui_hover, render))
            .add_event::<SpawnCellEvent>()
            .add_event::<RemoveCellEvent>();

        let args: Vec<String> = env::args().collect();

        for arg in args.iter() {
            match arg.as_str() {
                "cell_list" => {
                    app.add_systems(Update, cell_list_ui);
                }
                "world_inspector" => {
                    app.add_plugins(WorldInspectorPlugin::new());
                }
                "cell_count" => {
                    app.add_systems(Update, show_cell_count);
                }
                "is_empty" => {
                    app.add_systems(Update, check_is_empty_on_mouse_pos);
                }
                "chunk_gizmo" => {
                    app.add_systems(Update, chunk_gizmo);
                    info!("chunk gizmo enabled");
                }
                _ => info!("{}", ("this arg not supported ".to_owned() + arg.as_str())),
            }
        }
    }
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, asset_server: Res<AssetServer>) {
    commands.insert_resource(CellAssets {
        handles: HashMap::new(),
    });

    info!("loading config asset.....");
    let config_handle = asset_server.load::<ConfigAsset>("config.config");
    commands.spawn_empty().insert(config_handle);

    commands.spawn((
        Mesh2dHandle(meshes.add(shape::Quad::new(Vec2::new(10.0, 10.0)).into())),
        SpatialBundle::INHERITED_IDENTITY,
        NoAutomaticBatching,
        InstanceMaterialData(vec![InstanceData {
            position: Vec3::new(10.0, 10.0, 0.0),
            scale: 1.0,
            color: Color::hex("FF00FF").unwrap().into(),
        }]),
        // NOTE: Frustum culling is done based on the Aabb of the Mesh and the GlobalTransform.
        // As the cube is at the origin, if its Aabb moves outside the view frustum, all the
        // instanced cubes will be culled.
        // The InstanceMaterialData contains the 'GlobalTransform' information for this custom
        // instancing, and that is not taken into account with the built-in frustum culling.
        // We must disable the built-in frustum culling by adding the `NoFrustumCulling` marker
        // component to avoid incorrect culling.
        NoFrustumCulling,
    ));

    // camera
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 15.0),
            projection: OrthographicProjection {
                far: 1000.,
                near: -1000.,
                scale: 0.08,
                ..Default::default()
            },
            ..default()
        },
        MainCamera,
    ));
}

fn init_meshes(mut meshes: ResMut<Assets<Mesh>>, mut commands: Commands) {
    commands.insert_resource(CellMesh::from_world(meshes));
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
    mut cell_type: ResMut<CellTypeToSpawn>,
) {
    for (entity, handle) in query.iter() {
        if let Some(cell_asset) = cell_assets_storage.get(handle) {
            // Now you have access to cell_asset data like cell_type_name
            cell_assets
                .handles
                .insert(cell_asset.cell_type_name.clone(), handle.clone());
            info!("{:?}", cell_asset);
            // Remove the entity to avoid reprocessing
            cell_type.selected = Some(Selected {
                name: cell_asset.cell_type_name.clone(),
                handle: handle.clone(),
            });
            commands.entity(entity).despawn();
        }
    }
}
