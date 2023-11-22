use bevy::prelude::*;

use crate::{grid::{Grid, TileState, grid_to_space, GridEntity, GridScale}, asset_loading::BuildingAtlasHandle};

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {

    }
}


#[derive(Component)]
pub struct BuildingTag;

#[derive(Bundle)]
pub struct BuildingBundle {
    pub tag: BuildingTag,
    pub sprite: SpriteSheetBundle,
    pub grid_entity: GridEntity
}


pub fn spawn_building(
    commands: &mut Commands,
    grid: &mut Grid,
    grid_scale: &GridScale,
    location: IVec2,
    spawn_info: &(usize, String, IVec2),
    atlas_handle: &BuildingAtlasHandle
) {
    for x in 0..=spawn_info.2.x {
        for y in 0..=spawn_info.2.y {
            if grid[location + IVec2::new(x, y)] != TileState::Empty {return}
        }
    }
    let max = location + spawn_info.2;
    let grid_entity = GridEntity::new(location, Some(max));
    for cell in grid_entity.cells.iter() {
        grid.tiles.insert(*cell, TileState::Building);
    }
    let offset = Vec2::new(spawn_info.2.x as f32, spawn_info.2.y as f32) * Vec2::splat(grid.tile_size / 2.0);

    commands.spawn(BuildingBundle {
        tag: BuildingTag,
        sprite: SpriteSheetBundle {
            texture_atlas: atlas_handle.0.clone(),
            sprite: TextureAtlasSprite::new(spawn_info.0),
            transform: Transform {
                translation: (grid_to_space(location, grid) + offset).extend(0.0),
                scale: grid_scale.0,
                ..Default::default()
            },
            ..Default::default()
        },
        grid_entity: grid_entity
    })
    .insert(Name::new(spawn_info.1.clone()));
}