use crate::{
    assets::CellAsset, components::Cell, custom_renderer_plugin::{InstanceData, InstanceMaterialData}, resources::{cell_world::{self, CellWorld}, CellAssets}
};
use bevy::prelude::*;

pub fn render(
    cell_world: Res<CellWorld>,
    mut instance_material_data_entity: Query<&mut InstanceMaterialData>,
    query: Query<((&Transform, &Cell), With<Cell>)>,
    cell_assets_handles: Res<CellAssets>,
    cell_assets: Res<Assets<CellAsset>>,
) {
    let mut cells_material_data = instance_material_data_entity.single_mut();
    cells_material_data.0 = vec![];

    for chunk in cell_world.chunks.iter() {
        for cell_option in chunk.1.cells {
            match cell_option {
                Some(cell_entity) => {
                    match query.get(cell_entity) {
                        Ok(((transform, cell), _)) => {
                            match cell_assets_handles.handles.get(&cell.cell_type) {
                                Some(cell_asset_handle) => {
                                    match cell_assets.get(cell_asset_handle) {
                                        Some(cell_asset) => {
                                            cells_material_data.0.push(InstanceData {
                                                position: transform.translation,
                                                scale: 1f32,
                                                color: cell_asset.color.into(),
                                            })
                                        },
                                        None => {continue},
                                    }
                                    
                                },
                                None => {continue},
                            }
                            
                        },
                        Err(_) => {continue;},
                    } 
                },
                None => {continue},
            }
            
        }
    }
}
