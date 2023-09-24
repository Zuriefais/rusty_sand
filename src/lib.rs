pub mod setup;
pub mod lib {
    use bevy::prelude::*;
    use bevy::sprite::MaterialMesh2dBundle;
    use bevy::window::{PresentMode, PrimaryWindow};
    use bevy::winit::WinitWindows;
    use bevy::DefaultPlugins;
    use bevy_egui::{egui, EguiContexts, EguiPlugin};
    use winit::window::Icon;

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
    pub struct World {}

    pub struct SetupPlugin;

    impl Plugin for SetupPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, set_window_icon)
                .add_systems(Startup, setup)
                .insert_resource(ClearColor(Color::rgb(0.0, 0.170, 0.253)))
                .add_plugins(DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "rusty sand".into(),
                        resolution: (500., 300.).into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }))
                .add_plugins(EguiPlugin)
                .add_systems(Update, ui_example_system)
                .add_systems(Update, my_cursor_system)
                .add_systems(Update, spawn_cell_on_click);
        }
    }

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

        let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
        primary.set_window_icon(Some(icon));
    }

    fn ui_example_system(mut contexts: EguiContexts) {
        egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
            ui.label("world");
        });
    }

    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn(CursorPosition {
            pos: Vec2 { x: 0f32, y: 0f32 },
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
                    red: 0.194,
                    green: 0.178,
                    blue: 0.128,
                    alpha: 1.0,
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
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        buttons: Res<Input<MouseButton>>,
        cursor_positions: Query<&mut CursorPosition>,
    ) {
        if buttons.just_pressed(MouseButton::Left) {
            spawn_cell(
                commands,
                meshes,
                materials,
                cursor_positions.single().pos.extend(0f32),
            );
        }
    }

    fn spawn_cell(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        pos: Vec3,
    ) {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform {
                    translation: pos,
                    scale: CELL_SIZE,
                    ..Default::default()
                },
                material: materials.add(ColorMaterial::from(Color::Rgba {
                    red: 0.194,
                    green: 0.178,
                    blue: 0.128,
                    alpha: 1.0,
                })),
                ..default()
            },
            Cell {
                cell_type: CellType::Sand,
            },
        ));
    }
}
