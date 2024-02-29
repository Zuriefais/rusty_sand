use bevy::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

#[derive(Reflect, Component, Default, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Cell {
    pub cell_type: String,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
pub struct SlowdownCoefficient(pub f32);

#[derive(Component)]
pub struct Player;
