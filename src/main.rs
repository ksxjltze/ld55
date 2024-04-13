use bevy::{prelude::*, transform::commands, utils::HashMap, window::PrimaryWindow};
use std::ops::Index;

const GLOBAL_SCALE: f32 = 1.0;
const TILE_SIZE: f32 = 64.0;
const MUSHROOM_SPEED: f32 = 10.0;

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
struct Mushroom;

#[derive(Component)]
struct MushroomSummonManager {
    timer: f32,
    interval: f32,
}

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

fn setup_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/Roboto-Regular.ttf");
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.1, 0.5, 0.1).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button 1",
                        TextStyle {
                            font: font_handle.clone(),
                            font_size: 40.0,
                            // Alpha channel of the color controls transparency.
                            color: Color::rgba(1.0, 1.0, 1.0, 0.2),
                        },
                    ));
                });

            // Button with a different color,
            // to demonstrate the text looks different due to its transparency.
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.5, 0.1, 0.5).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button 2",
                        TextStyle {
                            font: font_handle.clone(),
                            font_size: 40.0,
                            // Alpha channel of the color controls transparency.
                            color: Color::rgba(1.0, 1.0, 1.0, 0.2),
                        },
                    ));
                });
        });
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

    commands.spawn(MushroomSummonManager {
        timer: 0.0,
        interval: 2.0,
    });
}

fn mushroom_summon_system(
    mut commands: Commands,
    image_manager: Res<ImageManager>,
    q_mushroom_base: Query<&Transform, With<MushroomBase>>,
    mut q_summon_manager: Query<&mut MushroomSummonManager>,
) {
    let mushroom_sprite = &image_manager[ImageType::Mushroom];
    let mushroom_base_position = q_mushroom_base.single().translation;

    let mut mushroom_summon_manager = q_summon_manager.single_mut();

    if mushroom_summon_manager.timer <= 0.0 {
        mushroom_summon_manager.timer = mushroom_summon_manager.interval;

        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(mushroom_base_position.x, mushroom_base_position.y, 0.0),
                    scale: (Vec3::splat(GLOBAL_SCALE)),
                    ..default()
                },
                texture: mushroom_sprite.handle(),
                ..default()
            },
            Mushroom,
        ));
    }
}

fn mushroom_attack_system(mut q_mushroom: Query<&mut Transform, With<Mushroom>>, time: Res<Time>) {
    q_mushroom.for_each_mut(|mut transform| {
        transform.translation.x += MUSHROOM_SPEED * time.delta_seconds();
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource::<ImageManager>(ImageManager {
            images: HashMap::new(),
        })
        .add_systems(PreStartup, load_assets_system)
        .add_systems(Startup, (setup_system, setup_ui_system))
        .add_systems(Update, (mushroom_summon_system, mushroom_attack_system))
        .run();
}
