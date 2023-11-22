use bevy::{prelude::*, asset::LoadedFolder, utils::HashMap};
use ron::from_str;
use std::fs;
use serde::Deserialize;
use crate::{AppState, interaction::InteractionSpriteIndices};

const BUILDING_SPRITE_PATH: &str = "robot_game/sprites/buildings";
const SELECTOR_SPRITE_PATH: &str = "robot_game/sprites/selector_images";


pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Setup), load_all_folders)
            .add_systems(Update, check_folders_loaded.run_if(in_state(AppState::Setup)))
            .add_systems(OnExit(AppState::Setup), (create_atlases, apply_deferred, create_building_selector_bindings).chain());
    }
}

#[derive(Resource)]
pub struct AssetFolderHandles {
    pub handles: Vec<Handle<LoadedFolder>>,
}
#[derive(Resource)]
pub struct LoadedFolderCount(usize);

#[derive(Resource)]
pub struct BuildingAtlasHandle(pub Handle<TextureAtlas>);

#[derive(Resource)]
pub struct WallAtlasHandle(pub Handle<TextureAtlas>);

#[derive(Resource)]
pub struct SelectionSpriteAtlasHandle(pub Handle<TextureAtlas>);

#[derive(Resource)]
pub struct RobotAtlasHandle(pub Handle<TextureAtlas>);

#[derive(Component)]
pub struct StateAnimationIndex {
    pub index: usize
}

#[derive(Component)]
pub struct StepableAnimation {
    pub current_index: usize,
    pub first: usize,
    pub len: usize,
}

impl StepableAnimation {
    pub fn step_forward(&mut self) {
        self.current_index = self.first + (self.current_index - self.first + 1) % self.len;
    }

    pub fn step_backward(&mut self) {
        if self.current_index == self.first {self.current_index = self.first + self.len - 1; return;}

        self.current_index -= 1;
    }
}


pub fn load_all_folders(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let mut folders = Vec::new();
    folders.push(asset_server.load_folder(BUILDING_SPRITE_PATH));
    folders.push(asset_server.load_folder(SELECTOR_SPRITE_PATH));
    commands.insert_resource(AssetFolderHandles {
        handles: folders,
    });
    commands.insert_resource(LoadedFolderCount(0));
}

pub fn check_folders_loaded(
    mut next_state: ResMut<NextState<AppState>>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
    folders: Res<AssetFolderHandles>,
    mut loaded_count: ResMut<LoadedFolderCount>
) {
    for event in events.read() {
        for folder in folders.handles.iter() {
            if event.is_loaded_with_dependencies(folder) {
                loaded_count.0 += 1;
            }
        }
    }
    if loaded_count.0 == folders.handles.len() {
        next_state.set(AppState::Finished)
    }
}


pub fn create_atlases(
    asset_server: Res<AssetServer>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    folder_handles: Res<AssetFolderHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut commands: Commands
) {
    create_wall_atlas(&asset_server, &mut texture_atlases, &mut commands);
    create_building_atlas(&loaded_folders, &folder_handles, &mut texture_atlases, &mut textures, &mut commands);
    create_info_sprites_atlas(&loaded_folders, &folder_handles, &mut texture_atlases, &mut textures, &mut commands);
    create_robot_atlas(&asset_server, &mut texture_atlases, &mut commands);
}


fn create_wall_atlas(
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    commands: &mut Commands
) {
    let walls_handle = asset_server.load("robot_game/sprite_sheets/walls.png");
    let walls_atlas = TextureAtlas::from_grid(walls_handle, Vec2::new(50.0, 50.0), 4, 4, None, None);
    commands.insert_resource(WallAtlasHandle(texture_atlases.add(walls_atlas)));
}


fn create_building_atlas(
    loaded_folders: &Assets<LoadedFolder>,
    folder_handles: &AssetFolderHandles,
    texture_atlases: &mut Assets<TextureAtlas>,
    textures: &mut Assets<Image>,
    commands: &mut Commands
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    let sprite_folder_handles = loaded_folders.get(&folder_handles.handles[0]).unwrap();
    for handle in sprite_folder_handles.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };

        texture_atlas_builder.add_texture(id, texture);
    }

    let building_atlas = texture_atlas_builder.finish(textures).unwrap();
    // let texture_atlas_texture = building_atlas.texture.clone();
    commands.insert_resource(BuildingAtlasHandle(texture_atlases.add(building_atlas)));
    // commands.spawn(SpriteBundle {
    //     texture: texture_atlas_texture,
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });
}


fn create_info_sprites_atlas(
    loaded_folders: &Assets<LoadedFolder>,
    folder_handles: &AssetFolderHandles,
    texture_atlases: &mut Assets<TextureAtlas>,
    textures: &mut Assets<Image>,
    commands: &mut Commands
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    let loaded_folder = loaded_folders.get(&folder_handles.handles[1]).unwrap();
    for handle in loaded_folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };

        texture_atlas_builder.add_texture(id, texture);
    }

    let building_atlas = texture_atlas_builder.finish(textures).unwrap();
    commands.insert_resource(SelectionSpriteAtlasHandle(texture_atlases.add(building_atlas)));
}

fn create_robot_atlas(
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    commands: &mut Commands
) {
    let robot_handle = asset_server.load("robot_game/sprite_sheets/robot.png");
    let robot_atlas = TextureAtlas::from_grid(robot_handle, Vec2::new(50.0, 50.0), 3, 1, None, None);
    commands.insert_resource(RobotAtlasHandle(texture_atlases.add(robot_atlas)));
}




#[derive(Deserialize)]
struct BuildingInfo {
    name: String,
    world_sprite: String,
    ui_sprite: String,
    size: [usize; 2]
}

#[derive(Resource)]
pub struct BuildingBindings(pub HashMap<usize, (usize, String, IVec2)>);

pub fn create_building_selector_bindings(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    building_atlas: Res<BuildingAtlasHandle>,
    select_atlas: Res<SelectionSpriteAtlasHandle>,
) {
    let building_bindings_str = fs::read_to_string("assets/building_bindings.ron").unwrap();

    let building_bindings: Vec<BuildingInfo> = from_str(&building_bindings_str).unwrap_or_else(|e| {
        println!("Failed to load config for graphics: {}", e);
        std::process::exit(1);
    });

    

    let mut bindings_map = HashMap::new();
    let building_atlas = texture_atlases.get(&building_atlas.0).unwrap();
    let ui_atlas = texture_atlases.get(&select_atlas.0).unwrap();
    // println!("{:?}", building_atlas);

    for binding in building_bindings {
        let world_sprite_handle = asset_server.get_handle([BUILDING_SPRITE_PATH, &binding.world_sprite].join("/")).unwrap();
        let ui_sprite_handle = asset_server.get_handle([SELECTOR_SPRITE_PATH, &binding.ui_sprite].join("/")).unwrap();
        let world_sprite_index = building_atlas.get_texture_index(world_sprite_handle).unwrap();
        let ui_sprite_index = ui_atlas.get_texture_index(ui_sprite_handle).unwrap();
        bindings_map.insert(ui_sprite_index, (world_sprite_index, binding.name, IVec2::new(binding.size[0] as i32 - 1, binding.size[1] as i32 - 1)));
    }

    commands.insert_resource(BuildingBindings(bindings_map));

    commands.insert_resource(InteractionSpriteIndices {
        delete: ui_atlas.get_texture_index(asset_server.get_handle([SELECTOR_SPRITE_PATH, "Delete.png"].join("/")).unwrap()).unwrap(),
        wall: ui_atlas.get_texture_index(asset_server.get_handle([SELECTOR_SPRITE_PATH, "Wall.png"].join("/")).unwrap()).unwrap()
    });

}