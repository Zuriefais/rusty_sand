use crate::{
    components::{Cell, Player},
    custom_renderer_plugin::{InstanceData, InstanceMaterialData},
    resources::{cell_world::CellWorld, CellAssets},
};
use bevy::prelude::*;

pub fn render(
    cell_world: Res<CellWorld>,
    mut instance_material_data_entity: Query<&mut InstanceMaterialData>,
    query: Query<((&Transform, &Cell), With<Cell>)>,
    cell_assets_handles: Res<CellAssets>,
    player_query: Query<&Transform, With<Player>>,
) {
    let mut cells_material_data = instance_material_data_entity.single_mut();
    cells_material_data.0.clear();
    cells_material_data.0.push(InstanceData {
        position: Vec3::new(10.0, 10.0, 0.0),
        scale: 1.0,
        color: Color::hex("FF00FF").unwrap().into(),
    });

    for chunk in cell_world.chunks.iter() {
        for cell_option in &chunk.1.cells {
            if let Some(cell_entity) = cell_option {
                if let Ok(((transform, cell), _)) = query.get(*cell_entity) {
                    match cell_assets_handles
                        .get_color_by_name(cell.cell_type.clone())
                        .clone()
                    {
                        Some(color) => {
                            cells_material_data.0.push(InstanceData {
                                position: transform.translation,
                                scale: 1.0,
                                color: color.into(),
                            });
                        }
                        None => todo!(),
                    }
                }
            }
        }
    }
    let player_pos = player_query.single().translation;

    cells_material_data.0.push(InstanceData {
        position: player_pos,
        scale: 1.0,
        color: Color::hex("#19212e").unwrap().into(),
    });

    // info!("{}", cells_material_data.0.len());
}
