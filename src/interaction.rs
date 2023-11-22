use bevy::prelude::*;

use crate::{AppState, grid::{Grid, grid_to_space, space_to_grid, GridEntity, delete_grid_entity, GridScale}, walls::spawn_wall, asset_loading::{WallAtlasHandle, StepableAnimation, SelectionSpriteAtlasHandle, BuildingAtlasHandle, BuildingBindings}, building::spawn_building};


pub struct TileSelectPlugin;

impl Plugin for TileSelectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Finished), spawn_tile_select_indicator)
            .add_systems(Update, (tile_select_follow_cursor, update_interaction_selection).run_if(in_state(AppState::Finished)));
    }
}


#[derive(Component)]
pub struct TileSelectIndicator {
    pub pos: IVec2
}

pub fn spawn_tile_select_indicator(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_scale: Res<GridScale>
) {
    let indicator_sprite_handle = asset_server.load("robot_game/sprites/misc/Tile_Select.png");
    commands.spawn(
        SpriteBundle {
            texture: indicator_sprite_handle,
            transform: Transform {
                translation: Vec3::ZERO,
                scale: grid_scale.0,
                ..Default::default()
            },
            ..Default::default()
        }
    )
    .insert(TileSelectIndicator{pos: IVec2::splat(0)});
}

pub fn tile_select_follow_cursor(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut indicator_query: Query<(&mut Transform, &mut TileSelectIndicator)>,
    grid: Res<Grid>
) {
    let (cam, cam_transform) = camera_query.single();
    let window = window_query.single();
    let (mut indicator_transform, mut indicator_grid_pos) = indicator_query.single_mut();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| cam.viewport_to_world(cam_transform, cursor))
        .map(|ray| ray.origin.truncate()) {
            indicator_grid_pos.pos = space_to_grid(world_position.extend(0.0), &grid);
            indicator_transform.translation = grid_to_space(indicator_grid_pos.pos, &grid).extend(1.0);
        }
}

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Finished), spawn_interaction_selection)
            .add_systems(Update, interaction.run_if(in_state(AppState::Finished)));
    }
}



pub fn spawn_interaction_selection(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    texture_atlas: Res<SelectionSpriteAtlasHandle>,
    indices: Res<InteractionSpriteIndices>
) {
    let info_square_handle = asset_server.load("robot_game/sprites/misc/Info_Square.png");

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(0.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::End,
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(ImageBundle {
            z_index: ZIndex::Local(1),
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(100.),
                height: Val::Px(100.),
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                ..Default::default()
            },
            image: UiImage{
                texture: info_square_handle,
                ..Default::default()
            },
            ..Default::default()
        });
        parent.spawn(AtlasImageBundle {
            z_index: ZIndex::Local(2),
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(70.),
                height: Val::Px(70.),
                bottom: Val::Px(15.0),
                left: Val::Px(15.0),
                ..Default::default()
            },
            texture_atlas: texture_atlas.0.clone(),
            texture_atlas_image: UiTextureAtlasImage {
                index: indices.delete,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(StepableAnimation {
            current_index: 0,
            first: 0,
            len: 6
        });
    });
}

pub fn update_interaction_selection(
    keyboard: Res<Input<KeyCode>>,
    mut interaction_selection_sprite: Query<(&mut StepableAnimation, &mut UiTextureAtlasImage)>
) {
    let (mut animation_index, mut image) = interaction_selection_sprite.single_mut();
    if keyboard.just_pressed(KeyCode::BracketRight) {
        animation_index.step_forward();
    } else if keyboard.just_pressed(KeyCode::BracketLeft) {
        animation_index.step_backward();
    }

    image.index = animation_index.current_index;
}

#[derive(Resource, Debug)]
pub struct InteractionSpriteIndices {
    pub delete: usize,
    pub wall: usize,
}

pub fn interaction(
    mouse: Res<Input<MouseButton>>,
    tile_select: Query<&TileSelectIndicator>,
    info_index: Query<&StepableAnimation>,
    interaction_indices: Res<InteractionSpriteIndices>,
    building_bindings: Res<BuildingBindings>,

    mut commands: Commands,
    mut grid: ResMut<Grid>,
    grid_scale: Res<GridScale>,
    wall_atlas: Res<WallAtlasHandle>,
    grid_entity_query: Query<(&GridEntity, Entity)>,
    building_atlas: Res<BuildingAtlasHandle>,

    
) {
    let tile_pos = tile_select.single();
    let index = info_index.single();

    if mouse.pressed(MouseButton::Left) {
        if index.current_index == interaction_indices.delete {
            delete_grid_entity(&mut commands, &mut grid, tile_pos.pos, &grid_entity_query);
        } else if index.current_index == interaction_indices.wall {
            spawn_wall(&mut commands, &mut grid, &grid_scale, tile_pos.pos, &wall_atlas);
        } else {
            if let Some(spawn_info) = building_bindings.0.get(&index.current_index) {
                // println!("{}", index.current_index);
                // println!("{}", spawn_info.1);

                spawn_building(&mut commands, &mut grid, &grid_scale, tile_pos.pos, spawn_info, &building_atlas);
            }
        }


    }
}