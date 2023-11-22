use bevy::prelude::*;

use crate::{a_star::a_star, grid::{Grid, grid_to_space, TileState, GridScale}, AppState};


const START: IVec2 = IVec2{x: -8, y: -2};
const END: IVec2 = IVec2{x: 7, y: 3};

pub struct PathFindTestPlugin;

impl Plugin for PathFindTestPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Finished), setup)
            .add_systems(Update, run_path_finding.run_if(in_state(AppState::Finished)));
    }
}


#[derive(Component)]
pub struct PathTag;

#[derive(Resource)]
pub struct PathFindSprites {
    pub start: Handle<Image>,
    pub end: Handle<Image>,
    pub other: Handle<Image>
}
#[derive(Resource)]
pub struct PathFindCount(usize);


pub fn setup(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    grid_scale: Res<GridScale>,
    asset_server: ResMut<AssetServer>,
) {

    grid.tiles.insert(START, TileState::InteractionPoint);
    grid.tiles.insert(END, TileState::InteractionPoint);

    let start_sprite = asset_server.load("robot_game/sprites/misc/PathFindTestStart.png");
    let end_sprite = asset_server.load("robot_game/sprites/misc/PathFindTestEnd.png");
    let path_sprite = asset_server.load("robot_game/sprites/misc/PathFindTestPath.png");

    commands.spawn(SpriteBundle {
        texture: start_sprite.clone(),
        transform: Transform {
            translation: grid_to_space(START, &grid).extend(0.0),
            scale: grid_scale.0,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: end_sprite.clone(),
        transform: Transform {
            translation: grid_to_space(END, &grid).extend(0.0),
            scale: grid_scale.0,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.insert_resource(PathFindSprites {
        start: start_sprite,
        end: end_sprite,
        other: path_sprite
    });

    commands.insert_resource(PathFindCount(1));
}


pub fn run_path_finding(
    keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
    paths: Query<Entity, With<PathTag>>,
    grid: Res<Grid>,
    grid_scale: Res<GridScale>,
    sprites: Res<PathFindSprites>,
    mut count: ResMut<PathFindCount>
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        for entity in paths.iter() {
            commands.entity(entity).despawn_recursive();
        };
        if let Some(points) = a_star(START, END, &grid, count.0) {
            for i in 1..points.len() - 1 {
                commands.spawn(SpriteBundle {
                    texture: sprites.other.clone(),
                    transform: Transform {
                        translation: grid_to_space(points[i], &grid).extend(-1.0),
                        scale: grid_scale.0,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(PathTag);
            }
            count.0 += 1;
        }
    }
}