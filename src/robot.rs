use bevy::prelude::*;

use crate::{asset_loading::RobotAtlasHandle, grid::{grid_to_space, Grid, GridScale}, AppState};


pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Finished), spawn_robot)
            .add_systems(Update, update_robot_sprites.run_if(in_state(AppState::Finished)));
    }
}



#[derive(Component)]
pub enum RobotState {
    Idle,
    Running,
    Stuck
}

#[derive(Component)]
pub struct PathFollower {
    pub path: Vec<IVec2>,
    pub target: IVec2
}

impl Default for PathFollower {
    fn default() -> Self {
        PathFollower {
            path: Vec::new(),
            target: IVec2::ZERO
        }
    }
}

#[derive(Component)]
pub struct Robot {
    pub location: IVec2,
}


#[derive(Bundle)]
pub struct RobotBundle {
    pub sprite: SpriteSheetBundle,
    pub robot: Robot,
    pub path_follow: PathFollower,
    pub brain_state: RobotState
}

pub fn spawn_robot(
    mut commands: Commands,
    atlas: Res<RobotAtlasHandle>,
    grid: Res<Grid>,
    grid_scale: Res<GridScale>
) {
    commands.spawn( RobotBundle {
        sprite: SpriteSheetBundle {
            texture_atlas: atlas.0.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform {
                translation: grid_to_space(IVec2::ZERO, &grid).extend(0.0),
                scale: grid_scale.0,
                ..Default::default()
            },
            ..Default::default()
        },
        robot: Robot {
            location: IVec2::ZERO
        },
        path_follow: PathFollower::default(),
        brain_state: RobotState::Stuck
    }
    );
}


pub fn update_robot_sprites(
    mut sprite_query: Query<(&RobotState, &mut TextureAtlasSprite)>
) {
    for (brain_state, mut sprite) in sprite_query.iter_mut() {
        match brain_state {
            RobotState::Idle => {sprite.index = 1},
            RobotState::Running => {sprite.index = 0},
            RobotState::Stuck => {sprite.index = 2}
        }
    }
}



