use bevy::{prelude::*, utils::HashMap, window::PrimaryWindow};
use std::ops::Index;

const GLOBAL_SCALE: f32 = 2.0;
const TILE_SIZE: f32 = 64.0;

#[derive(Eq, Hash, PartialEq)]
enum ImageType {
    Mushroom,
    Hero,
    MushroomBase,
    HeroBase,
    Ground,
}

#[derive(Resource)]
struct ImageManager {
    images: HashMap<ImageType, Sprite>,
}

struct Sprite {
    image_handle: Handle<Image>,
    width: u32,
    height: u32,
}

impl Sprite {
    fn handle(&self) -> Handle<Image> {
        return self.image_handle.clone_weak();
    }
}

impl ImageManager {
    fn get(&self, key: ImageType) -> &Sprite {
        return &self.images[&key];
    }
}

impl Index<ImageType> for ImageManager {
    type Output = Sprite;
    fn index(&self, key: ImageType) -> &Sprite {
        self.get(key)
    }
}

#[derive(Component)]
struct GameCamera;

#[derive(Component)]
struct MushroomBase;

#[derive(Component)]
struct Ground;

fn load_assets_system(mut image_manager: ResMut<ImageManager>, asset_server: Res<AssetServer>) {
    let mushroom_sprite_asset: Handle<Image> = asset_server.load("boi.png");
    let mushroom_base_sprite_asset: Handle<Image> = asset_server.load("base.png");
    let ground_sprite_asset: Handle<Image> = asset_server.load("ground.png");

    image_manager.images.insert(
        ImageType::Mushroom,
        Sprite {
            image_handle: mushroom_sprite_asset,
            width: 0,
            height: 0,
        },
    );

    image_manager.images.insert(
        ImageType::MushroomBase,
        Sprite {
            image_handle: mushroom_base_sprite_asset,
            width: 0,
            height: 0,
        },
    );

    image_manager.images.insert(
        ImageType::Ground,
        Sprite {
            image_handle: ground_sprite_asset,
            width: 0,
            height: 0,
        },
    );
}

fn setup_system(
    mut commands: Commands,
    image_manager: Res<ImageManager>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let mushroom_base_sprite = &image_manager[ImageType::MushroomBase];
    let ground_sprite = &image_manager[ImageType::Ground];

    let window = q_windows.single();
    let width = window.width();
    let height = window.height();

    let tile_x_count = (width / TILE_SIZE) as i32;
    let tile_y_count = ((height / TILE_SIZE) / 2.0) as i32;

    commands.spawn((Camera2dBundle::default(), GameCamera));

    let tile_size = TILE_SIZE as i32;
    let x_offset = width / 2.0;
    let y_offset = height / 2.0;

    for i in 0..tile_x_count + 1 {
        for j in 0..tile_y_count {
            let x = (i * tile_size) as f32 - x_offset;
            let y = (j * tile_size) as f32 - y_offset;

            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(x, y, 0.0),
                        scale: (Vec3::splat(1.0)),
                        ..default()
                    },
                    texture: ground_sprite.handle(),
                    ..default()
                },
                Ground,
            ));
        }
    }

    let initial_height = -y_offset + (tile_y_count as f32) * TILE_SIZE;
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-x_offset + TILE_SIZE, initial_height, 0.0),
                scale: (Vec3::splat(GLOBAL_SCALE)),
                ..default()
            },
            texture: mushroom_base_sprite.handle(),
            ..default()
        },
        MushroomBase,
    ));
}

fn test() {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource::<ImageManager>(ImageManager {
            images: HashMap::new(),
        })
        .add_systems(PreStartup, load_assets_system)
        .add_systems(Startup, setup_system)
        .add_systems(Update, test)
        .run();
}
