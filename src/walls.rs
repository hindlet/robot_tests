use bevy::prelude::*;

use crate::{AppState, grid::{grid_to_space, Grid, TileState, GridEntity, GridScale}, asset_loading::{StateAnimationIndex, WallAtlasHandle}};

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (update_wall_sprites, update_walls).run_if(in_state(AppState::Finished)));
    }
}



#[derive(Component)]
pub struct WallTag;


#[derive(Bundle)]
pub struct WallBundle {
    pub tag: WallTag,
    pub state: StateAnimationIndex,
    pub sprite: SpriteSheetBundle,
    pub grid_entity: GridEntity
}




pub fn update_wall_sprites(
    mut wall_query: Query<(&StateAnimationIndex, &mut TextureAtlasSprite), With<WallTag>>
) {
    for (state, mut sprite) in wall_query.iter_mut() {
        sprite.index = state.index;
    }
}

pub fn update_walls(
    mut wall_query: Query<(&GridEntity, &mut StateAnimationIndex), With<WallTag>>,
    grid: Res<Grid>,
) {
    for (grid_entity, mut state) in wall_query.iter_mut() {
        let index = grid_entity.min;
        let mut state_index = 0;

        if grid[index + IVec2::X] == TileState::Wall {state_index += 1};
        if grid[index + IVec2::Y] == TileState::Wall {state_index += 2};
        if grid[index - IVec2::X] == TileState::Wall {state_index += 4};
        if grid[index - IVec2::Y] == TileState::Wall {state_index += 8};

        state.index = state_index;
    }
}

pub fn spawn_wall(
    commands: &mut Commands,
    grid: &mut Grid,
    grid_scale: &GridScale,
    location: IVec2,
    atlas_handle: &WallAtlasHandle
) {
    if grid[location] != TileState::Empty {return}

    commands.spawn(WallBundle {
        tag: WallTag,
        state: StateAnimationIndex{index: 0},
        sprite: SpriteSheetBundle {
            texture_atlas: atlas_handle.0.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform {
                translation: grid_to_space(location, grid).extend(0.0),
                scale: grid_scale.0,
                ..Default::default()
            },
            ..Default::default()
        },
        grid_entity: GridEntity::new(location, None)
    })
    .insert(Name::new("Wall"));
    grid.tiles.insert(location, TileState::Wall);
}