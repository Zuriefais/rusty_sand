use crate::{
    components::Player,
    custom_renderer_plugin::{InstanceData, InstanceMaterialData},
    enums::{CELL_SIZE, CHUNK_SIZE},
    resources::{cell_world::CellWorld, CellAssets},
};
use bevy::prelude::*;
use rayon::prelude::*;

pub fn render(
    world: Res<CellWorld>,
    mut instance_material_data_entity: Query<&mut InstanceMaterialData>,
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

    let player_pos = player_query.single().translation;

    for (pos, _) in world.chunks.iter() {
        cells_material_data.append(&mut render_chunk(
            &world,
            pos.clone() * CHUNK_SIZE,
            &cell_assets_handles,
        ));
    }

    cells_material_data.0.push(InstanceData {
        position: player_pos,
        scale: 1.0,
        color: Color::hex("#19212e").unwrap().into(),
    });

    // info!("{}", cells_material_data.0.len());
}

fn render_chunk(
    world: &CellWorld,
    chunk_pos: IVec2,
    cell_assets_handles: &Res<CellAssets>,
) -> Vec<InstanceData> {
    let mut material_data = vec![];
    let chunk_pos_local =
        (CellWorld::calculate_chunk_pos(chunk_pos) * CHUNK_SIZE * CELL_SIZE.xy().as_ivec2())
            .as_vec2();
    let chunk = world.get_chunk(chunk_pos);
    if let Some(chunk) = chunk {
        for y in 0..CHUNK_SIZE.y {
            for x in 0..CHUNK_SIZE.x {
                let cell_pos = IVec2 { x: x, y: y };
                if let Some(cell) = chunk.get(cell_pos) {
                    let color = match cell_assets_handles.get_color(cell.0) {
                        Some(color) => color,
                        None => Color::PURPLE,
                    }
                    .into();

                    material_data.push(InstanceData {
                        position: (((cell_pos.as_vec2() + cell.1) * CELL_SIZE.xy())
                            + chunk_pos_local)
                            .extend(0f32),
                        scale: 1.0,
                        color: color,
                    })
                }
            }
        }
    }
    material_data
}
