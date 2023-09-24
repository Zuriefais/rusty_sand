use bevy::prelude::*;
use rusty_sand::lib::SetupPlugin;

fn main() {
    App::new().add_plugins(SetupPlugin).run();
}
