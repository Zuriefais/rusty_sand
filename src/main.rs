// main.rs or lib.rs
mod assets;
mod components;
mod enums;
mod events;
mod resources;
mod setup;
mod systems;
mod utils;
mod custom_renderer_plugin;

extern crate grid;

use bevy::prelude::*;
use setup::SetupPlugin;

fn main() {
    App::new().add_plugins(SetupPlugin).run();
}
