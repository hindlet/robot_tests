use bevy::{prelude::*, utils::HashMap};
use core::ops::Index;
use crate::AppState;


const SPRITE_TILE_SIZE: f32 = 50.0;
pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Setup), spawn_grid);
    }
}

#[derive(Component)]
pub struct GridEntity {
    pub min: IVec2,
    pub max: IVec2,
    pub cells: Vec<IVec2>
}

impl GridEntity {
    pub fn contains_cell(&self, cell: IVec2) -> bool {
        cell.x >= self.min.x && cell.y >= self.min.y && cell.x <= self.max.x && cell.y <= self.max.y
    }

    pub fn new(min: IVec2, max: Option<IVec2>) -> Self {
        if let Some(max_point) = max {
            let mut cells = Vec::new();
            for x in min.x..=max_point.x {
                for y in min.y..=max_point.y {
                    cells.push(IVec2::new(x, y));
                }
            }
            return GridEntity {
                min,
                max: max_point,
                cells: cells
            };
        } else {
            return GridEntity {
                min,
                max: min,
                cells: vec![min]
            };
        }
    }

}




#[derive(PartialEq)]
pub enum TileState {
    Empty,
    Wall,
    Building,
    Robot,
    InteractionPoint,
}


#[derive(Resource)]
pub struct Grid {
    pub tiles: HashMap<IVec2, TileState>,
    pub centre: Vec2,
    pub tile_size: f32
}

#[derive(Resource)]
pub struct GridScale(pub Vec3);

pub fn spawn_grid(
    mut commands: Commands
) {
    let tile_size = 25.0;
    commands.insert_resource(Grid{
        tiles: HashMap::new(),
        centre: Vec2::ZERO,
        tile_size: tile_size
    });
    commands.insert_resource(GridScale(Vec3::splat(tile_size / SPRITE_TILE_SIZE)));
}

impl Index<IVec2> for Grid {
    type Output = TileState;

    fn index(&self, index: IVec2) -> &Self::Output {
        if let Some(state) = self.tiles.get(&index) {
            return &state;
        } else {return &TileState::Empty;}
    }
}

pub fn grid_to_space(grid_location: IVec2, grid: &Grid) -> Vec2 {
    let x_offset = grid_location.x as f32 * grid.tile_size;
    let y_offset = grid_location.y as f32 * grid.tile_size;

    grid.centre + Vec2::new(x_offset, y_offset)
}

pub fn space_to_grid(space_location: Vec3, grid: &Grid) -> IVec2 {
    let x = ((space_location.x - grid.centre.x + grid.tile_size * 0.5) / grid.tile_size).floor() as i32;
    let y = ((space_location.y - grid.centre.y + grid.tile_size * 0.5) / grid.tile_size).floor() as i32;

    IVec2::new(x, y)
}

pub fn delete_grid_entity(
    commands: &mut Commands,
    grid: &mut Grid,
    location: IVec2,
    grid_entity_query: &Query<(&GridEntity, Entity)>
) {
    for (grid_entity, entity) in grid_entity_query.iter() {
        if grid_entity.contains_cell(location) {

            for cell in grid_entity.cells.iter() {
                grid.tiles.insert(*cell, TileState::Empty);
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}


#[cfg(test)]
mod space_to_grid_tests {
    use super::{Grid, space_to_grid, Vec2, IVec2, Vec3, HashMap};
    #[test]
    fn positive_tests() {
        let grid = Grid {
            tiles: HashMap::new(),
            centre: Vec2::ZERO,
            tile_size: 1.0
        };
        let index = space_to_grid(Vec3::new(-0.25, -0.25, 0.0), &grid);
        assert_eq!(index, IVec2::new(0, 0));
        let index = space_to_grid(Vec3::new(1.0, 0.0, 0.0), &grid);
        assert_eq!(index, IVec2::new(1, 0));
        let index = space_to_grid(Vec3::new(1.0, 1.5, 0.0), &grid);
        assert_eq!(index, IVec2::new(1, 2));

        let grid = Grid {
            tiles: HashMap::new(),
            centre: Vec2::ZERO,
            tile_size: 0.5
        };
        let index = space_to_grid(Vec3::new(-0.2, -0.2, 0.0), &grid);
        assert_eq!(index, IVec2::new(0, 0));
        let index = space_to_grid(Vec3::new(1.5, 0.5, 0.0), &grid);
        assert_eq!(index, IVec2::new(3, 1));
    }

    #[test]
    fn negative_tests() {
        let grid = Grid {
            tiles: HashMap::new(),
            centre: Vec2::ZERO,
            tile_size: 1.0
        };
        let index = space_to_grid(Vec3::new(-0.6, -0.5, 0.0), &grid);
        assert_eq!(index, IVec2::new(-1, 0));
        let index = space_to_grid(Vec3::new(-3.0, 2.0, 0.0), &grid);
        assert_eq!(index, IVec2::new(-3, 2));
        let index = space_to_grid(Vec3::new(-1.0, -1.5, 0.0), &grid);
        assert_eq!(index, IVec2::new(-1, -1));

        let grid = Grid {
            tiles: HashMap::new(),
            centre: Vec2::ZERO,
            tile_size: 0.5
        };
        let index = space_to_grid(Vec3::new(-1.2, -1.2, 0.0), &grid);
        assert_eq!(index, IVec2::new(-2, -2));
        let index = space_to_grid(Vec3::new(-1.5, -0.25, 0.0), &grid);
        assert_eq!(index, IVec2::new(-3, 0));
    }
}
