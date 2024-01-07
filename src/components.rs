use bevy::prelude::*;
use bevy_inspector_egui::{InspectorOptions, inspector_options::ReflectInspectorOptions};

#[derive(Reflect, Component, Default, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Cell {
    pub cell_type: String,
}

#[derive(Component)]
pub struct MainCamera;
