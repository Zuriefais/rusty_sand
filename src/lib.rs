pub mod setup;
pub mod lib {
    use bevy::log::LogPlugin;
    use bevy::prelude::*;
    use bevy::sprite::MaterialMesh2dBundle;
    use bevy::utils::HashMap;
    use bevy::window::{PresentMode, PrimaryWindow};
    use bevy::winit::WinitWindows;
    use bevy::DefaultPlugins;
    use bevy_egui::{egui, EguiContexts, EguiPlugin};
    use bevy_inspector_egui::quick::WorldInspectorPlugin;
    use grid::*;
    use lazy_static::lazy_static;
    use winit::window::Icon;
    use winit::window::Icon as WinitIcon;


    #[derive(Component)]
    pub struct Cell {
        pub cell_type: CellType,
    }

    #[derive(Component)]
    pub struct CursorPosition {
        pub pos: Vec2,
    }

    #[derive(Component)]
    pub struct MainCamera;

    lazy_static! {
        static ref CELL_COLOR: HashMap<CellType, bevy::render::color::Color> = {
            let mut map = HashMap::new();
            map.insert(
                CellType::Sand,
                bevy::render::color::Color::hex("f6d7b0").unwrap(),
            );
            map.insert(
                CellType::Stone,
                bevy::render::color::Color::hex("4E5754").unwrap(),
            );
            map
        };
    }

    #[derive(Eq, PartialEq, Hash, std::fmt::Debug, Clone, Copy)]
    pub enum CellType {
        Sand,
        Stone,
    }

    const CELL_SIZE: Vec3 = Vec3 {
        x: 10f32,
        y: 10f32,
        z: 10f32,
    };

    #[derive(Component)]
    pub struct CellTypeToSpawn {
        pub type_to_select: CellType,
    }

    #[derive(Resource)]
    pub struct World {
        pub grid: Grid<Entity>,
        pub cell_size: Vec3
    }

    impl World {
        pub fn default() -> World {
            World {
                grid: grid![],
                cell_size: Vec3 {
                    x: 10f32,
                    y: 10f32,
                    z: 10f32,
                },
            }
        }
    }

    pub struct SetupPlugin;

    impl Plugin for SetupPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, set_window_icon)
                .add_systems(Startup, setup)
                .insert_resource(ClearColor(Color::rgb(0.0, 0.170, 0.253)))
                .add_plugins(
                    DefaultPlugins
                        .set(WindowPlugin {
                            primary_window: Some(Window {
                                title: "rusty sand".into(),
                                resolution: (500., 300.).into(),
                                present_mode: PresentMode::AutoVsync,
                                fit_canvas_to_parent: true,
                                ..default()
                            }),
                            ..default()
                        })
                        .disable::<LogPlugin>(),
                )
                .add_plugins(EguiPlugin)
                .add_systems(Update, spawn_cell_type)
                .add_systems(Update, my_cursor_system)
                .add_systems(Update, spawn_cell_on_click)
                //.add_plugins(WorldInspectorPlugin::new())
                .add_systems(Update, physics)
                .add_systems(Update, move_camera);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_window_icon(
        main_window: Query<Entity, With<PrimaryWindow>>,
        windows: NonSend<WinitWindows>,
    ) {
        let Some(primary) = windows.get_window(main_window.single()) else {
            return;
        };
    
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open("icon.ico")
                .expect("Failed to open icon path")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
    
        let icon = WinitIcon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
        //primary.set_window_icon(Some(icon));
    }
    

    #[cfg(target_arch = "wasm32")]
    pub fn set_window_icon() {}

    fn spawn_cell_type(mut contexts: EguiContexts, mut query: Query<&mut CellTypeToSpawn>) {
        let mut selected = &query.single_mut().type_to_select.clone();
        egui::Window::new("cell type").show(contexts.ctx_mut(), |ui| {
            egui::ComboBox::from_label("Select one!")
                .selected_text(format!("{:?}", selected))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut selected, &CellType::Sand, "Sand");
                    ui.selectable_value(&mut selected, &CellType::Stone, "Stone");
                });
        });
        query.single_mut().type_to_select = selected.clone();
        // Use `ui.enum_select` to create the dropdown menu.
    }

    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
        commands.spawn(CursorPosition {
            pos: Vec2 { x: 0f32, y: 0f32 },
        });
        commands.spawn(CellTypeToSpawn {
            type_to_select: CellType::Sand,
        });
        commands.spawn((Camera2dBundle::default(), MainCamera));
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform {
                    translation: Vec3 {
                        x: -10f32,
                        y: 10f32,
                        z: 0f32,
                    },
                    scale: CELL_SIZE,
                    ..Default::default()
                },
                material: materials.add(ColorMaterial::from(Color::Rgba {
                    red: 0.246,
                    green: 0.215,
                    blue: 0.176,
                    alpha: 0.256,
                })),
                ..default()
            },
            Cell {
                cell_type: CellType::Sand,
            },
        ));
    }

    fn my_cursor_system(
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

    fn spawn_cell_on_click(
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<ColorMaterial>>,
        buttons: Res<Input<MouseButton>>,
        cursor_positions: Query<&mut CursorPosition>,
        query: Query<&CellTypeToSpawn>,
        cell_pos_query: Query<&Transform, With<Cell>>,
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
                new_cursor_position.extend(0f32),
                query.single().type_to_select,
                cell_pos_query,
            );
        }
    }

    fn spawn_cell_on_touch(
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<ColorMaterial>>,
        query: Query<&CellTypeToSpawn>,
        cell_pos_query: Query<&Transform, With<Cell>>,
        touches: Res<Touches>,
        windows: Query<&Window>,
        camera_q: Query<(&Transform, &Camera), With<MainCamera>>,
    ) {
        for finger in touches.iter() {
            if touches.just_pressed(finger.id()) {
                let touch_position = finger.position().clone();
                let mut new_touch_position = screen_to_world(touch_position, windows, camera_q);
                new_touch_position.x -= (new_touch_position.x as i32 % CELL_SIZE.x as i32) as f32;
                new_touch_position.y -= -(new_touch_position.y as i32 % CELL_SIZE.x as i32) as f32;

                spawn_cell(
                    commands,
                    meshes,
                    materials,
                    new_touch_position,
                    query.single().type_to_select,
                    cell_pos_query,
                );
                return;
            }
        }
    }

    fn screen_to_world(
        touch_position: Vec2,
        windows_query: Query<&Window>,
        cameras: Query<(&Transform, &Camera), With<MainCamera>>,
    ) -> Vec3 {
        let window = windows_query.iter().next().unwrap();

        // For the purpose of this example, we assume there's one main camera.
        // Adjust as necessary for your setup.
        let (camera_transform, camera) = cameras.iter().next().unwrap();

        // Screen to NDC
        let ndc = Vec3::new(
            (touch_position.x / window.width() as f32) * 2.0 - 1.0,
            (touch_position.y / window.height() as f32) * 2.0 - 1.0,
            0.5, // Middle of the near/far plane
        );

        // NDC to world space
        let world_position =
            camera.projection_matrix().inverse() * Vec4::new(ndc.x, ndc.y, ndc.z, 1.0);

        (camera_transform.compute_matrix() * world_position).truncate()
    }

    fn spawn_cell(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        pos: Vec3,
        cell_type: CellType,
        cell_pos_query: Query<&Transform, With<Cell>>,
    ) {
        for cell_pos in &cell_pos_query {
            if cell_pos.translation == pos {
                return;
            }
        }

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform {
                    translation: pos,
                    scale: CELL_SIZE,
                    ..Default::default()
                },
                material: materials.add(ColorMaterial::from(CELL_COLOR[&cell_type])),
                ..default()
            },
            Cell {
                cell_type: cell_type,
            },
        ));
    }

    fn physics(mut cells_query: Query<(Entity, &mut Cell, &mut Transform)>) {
        let entities: Vec<Entity> = cells_query.iter_mut().map(|(ent, _, _)| ent).collect();

        let mut to_move = Vec::new(); // Vec to track which entities need to be moved

        for i in 0..entities.len() {
            if let Ok((_, cell, transform)) = &cells_query.get(entities[i]) {
                // Note: not getting mutably here
                match cell.cell_type {
                    CellType::Sand => {
                        let mut stop = false;

                        for i2 in 0..entities.len() {
                            if let Ok((_, cell2, transform2)) = cells_query.get(entities[i2]) {
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

    fn move_camera(
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

        camera_transform.translation +=
            (move_dir * speed as f32 * time.delta_seconds()).extend(0f32);
    }
}
