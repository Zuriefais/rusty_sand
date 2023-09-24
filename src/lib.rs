pub mod setup;
pub mod lib {
    use bevy::prelude::*;
    use bevy::sprite::MaterialMesh2dBundle;
    use bevy::window::{PresentMode, PrimaryWindow};
    use bevy::winit::WinitWindows;
    use bevy::DefaultPlugins;
    use bevy_egui::{EguiPlugin, EguiContexts, egui};
    use winit::window::Icon;

    #[derive(Component)]
    pub struct Cell {
        cell_type: CellType
    }

    pub enum CellType {
        Sand,
        Stone
    }

    const CELL_SIZE: Vec3 = Vec3{x: 10f32, y: 10f32, z: 10f32 };

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
                },))
                .add_plugins(EguiPlugin)
                .add_systems(Update, ui_example_system);
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
        commands.spawn(Camera2dBundle::default());
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
            Cell { cell_type: CellType::Sand },
        ));
    }

    fn my_cursor_system(
        // need to get window dimensions
        windows: Res<Windows>,
        // query to get camera transform
        camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    ) {
        // get the camera info and transform
        // assuming there is exactly one main camera entity, so query::single() is OK
        let (camera, camera_transform) = camera_q.single();
    
        // get the window that the camera is displaying to (or the primary window)
        let window = if let RenderTarget::Window(id) = camera.target {
            windows.get(id).unwrap()
        } else {
            windows.get_primary().unwrap()
        };
    
        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            eprintln!("World coords: {}/{}", world_position.x, world_position.y);
        }
    }
}
