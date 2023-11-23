use bevy::prelude::*;

// Add your systems like set_window_icon, setup, spawn_cell_type, etc.
#[cfg(not(target_arch = "wasm32"))]
pub fn set_window_icon(
    _main_window: Query<Entity, With<bevy::window::PrimaryWindow>>,
    _windows: NonSend<bevy::winit::WinitWindows>,
) {

    //let Some(_primary) = windows.get_window(main_window.single()) else {
    //    return;
    //};

    //let (icon_rgba, icon_width, icon_height) = {
    //let image = image::open("icon.ico")
    //.expect("Failed to open icon path")
    //.into_rgba8();
    //let (width, height) = image.dimensions();
    //let rgba = image.into_raw();
    //(rgba, width, height)
    //};

    //let _icon = winit::window::Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    //primary.set_window_icon(Some(icon));
}

#[cfg(target_arch = "wasm32")]
pub fn set_window_icon() {}
