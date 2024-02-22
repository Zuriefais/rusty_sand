use crate::{
    assets::CellAsset,
    components::Cell,
    custom_renderer_plugin::{InstanceData, InstanceMaterialData},
    resources::{
        cell_world::{CellWorld},
        CellAssets,
    },
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
                    if let Some(cell_asset_handle) =
                        cell_assets_handles.handles.get(&cell.cell_type)
                    {
                        if let Some(cell_asset) = cell_assets.get(cell_asset_handle) {
                            cells_material_data.0.push(InstanceData {
                                position: transform.translation,
                                scale: 1.0,
                                color: cell_asset.color.into(),
                            });
                        }
                    }
                }
            }
        }
    }
    info!("{}", cells_material_data.0.len());
}
