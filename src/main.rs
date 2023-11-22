mod walls;
use walls::WallPlugin;
mod grid;
use grid::GridPlugin;
mod building;
use building::BuildingPlugin;
mod interaction;
use interaction::{TileSelectPlugin, InteractionPlugin};
mod asset_loading;
use asset_loading::AssetLoadingPlugin;
mod a_star;
mod pathfinding_testing;
use pathfinding_testing::PathFindTestPlugin;
mod robot;
use robot::RobotPlugin;
mod script;
mod item;


use bevy::{asset::LoadedFolder, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugins((WallPlugin, TileSelectPlugin, GridPlugin, BuildingPlugin, InteractionPlugin, AssetLoadingPlugin, RobotPlugin))
        .add_plugins(PathFindTestPlugin)
        .add_state::<AppState>()
        .add_systems(OnEnter(AppState::Setup), load_textures)
        .add_systems(Update, check_textures.run_if(in_state(AppState::Setup)))
        .add_systems(OnEnter(AppState::Finished), setup)
        .add_systems(Update, step_animations.run_if(in_state(AppState::Finished)))
        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
        .run();
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Setup,
    Finished,
}

#[derive(Resource, Default)]
struct SpriteFolder(Handle<LoadedFolder>);

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn step_animations(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}


fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load multiple, individual sprites from a folder
    commands.insert_resource(SpriteFolder(asset_server.load_folder("robot_game/sprites/misc")));
}

fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    sprite_folder: ResMut<SpriteFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    // Advance the `AppState` once all sprite handles have been loaded by the `AssetServer`
    for event in events.read() {
        if event.is_loaded_with_dependencies(&sprite_folder.0) {
            next_state.set(AppState::Finished);
        }
    }
}

fn setup(
    mut commands: Commands,
    // sprite_handles: Res<SpriteFolder>,

    // asset_server: Res<AssetServer>,
    // loaded_folders: Res<Assets<LoadedFolder>>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // mut textures: ResMut<Assets<Image>>,
) {
    // Build a `TextureAtlas` using the individual sprites
    // let mut texture_atlas_builder = TextureAtlasBuilder::default();
    // let loaded_folder = loaded_folders.get(&sprite_handles.0).unwrap();
    // for handle in loaded_folder.handles.iter() {
    //     let id = handle.id().typed_unchecked::<Image>();
    //     let Some(texture) = textures.get(id) else {
    //         warn!(
    //             "{:?} did not resolve to an `Image` asset.",
    //             handle.path().unwrap()
    //         );
    //         continue;
    //     };

    //     texture_atlas_builder.add_texture(id, texture);
    // }

    // let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    // let texture_atlas_texture = texture_atlas.texture.clone();

    // set up a scene to display our texture atlas
    commands.spawn(Camera2dBundle::default());
    // draw the atlas itself
    // commands.spawn(SpriteBundle {
    //     texture: texture_atlas_texture,
    //     transform: Transform {
    //         translation: Vec3::splat(0.0),
    //         scale: Vec3::splat(2.0),
    //         ..Default::default()
    //     },
    //     ..default()
    // });


    // let robot_handle = asset_server.load("robot_game/sprite_sheets/robot.png");
    // let robot_atlas = TextureAtlas::from_grid(robot_handle, Vec2::new(20.0, 19.0), 3, 1, None, None);
    // let robot_atlas_handle = texture_atlases.add(robot_atlas);
    // let robot_animation_indices = AnimationIndices { first: 0, last: 2 };
    // commands.spawn((
    //     SpriteSheetBundle {
    //         texture_atlas: robot_atlas_handle,
    //         sprite: TextureAtlasSprite::new(robot_animation_indices.first),
    //         transform: Transform {
    //             translation: Vec3::new(150.0, -150.0, 0.0),
    //             scale: Vec3::splat(5.0),
    //             ..Default::default()
    //         },
    //         ..default()
    //     },
    //     robot_animation_indices,
    //     AnimationTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
    // ));

    // let walls_handle = asset_server.load("robot_game/sprite_sheets/walls.png");
    // let walls_atlas = TextureAtlas::from_grid(walls_handle, Vec2::new(50.0, 50.0), 4, 4, None, None);
    // let walls_atlas_handle = texture_atlases.add(walls_atlas);
    // let walls_animation_indices = AnimationIndices { first: 0, last: 15 };
    // commands.spawn((
    //     SpriteSheetBundle {
    //         texture_atlas: walls_atlas_handle,
    //         sprite: TextureAtlasSprite::new(walls_animation_indices.first),
    //         transform: Transform {
    //             translation: Vec3::splat(0.0),
    //             scale: Vec3::splat(10.0),
    //             ..Default::default()
    //         },
    //         ..default()
    //     },
    //     walls_animation_indices,
    //     AnimationTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
    // ));


}