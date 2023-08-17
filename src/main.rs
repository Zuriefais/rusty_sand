use bevy::prelude::*;
use bevy::window::{Window, PresentMode};
use bevy::winit::WinitWindows;
use bevy::{prelude::*, window::PrimaryWindow};
use winit::window::Icon;

fn main() {
    App::new()
        .add_systems(Startup ,set_window_icon)
        .add_systems(Startup ,setup)
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
        .run();
}

pub fn set_window_icon(
    main_window: Query<Entity, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
) {
    let Some(primary) = windows.get_window(main_window.single()) else {return};

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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
