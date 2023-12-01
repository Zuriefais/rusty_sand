use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::MainCamera;

#[cfg(not(target_arch = "wasm32"))]
pub fn set_window_icon(
    main_window: Query<Entity, With<bevy::window::PrimaryWindow>>,
    windows: NonSend<bevy::winit::WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    use winit::window::Icon;

    let Some(_primary) = windows.get_window(main_window.single()) else {
        return;
    };

    let primary_entity = primary_window.single();
    let primary = windows.get_window(primary_entity).unwrap();

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

#[cfg(target_arch = "wasm32")]
pub fn set_window_icon() {}
