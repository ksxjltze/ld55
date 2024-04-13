use bevy::{prelude::*, transform::commands, utils::HashMap, window::PrimaryWindow};
use std::ops::Index;

const GLOBAL_SCALE: f32 = 1.0;
const TILE_SIZE: f32 = 64.0;
const INITIAL_SPORE_COUNT: i32 = 10;

const HERO_BASE_HP: f32 = 1000.0;
const HERO_BASE_ATK: f32 = 10.0;
const HERO_BASE_MOVE_SPEED: f32 = 20.0;
const HERO_BASE_ATK_SPEED: f32 = 1.0;
const HERO_BASE_ATK_RANGE: f32 = 50.0;
const HERO_BASE_LEVEL: i32 = 1;
const HERO_BASE_EXP_REQUIRED: f32 = 100.0;

const MUSHROOM_BASE_HP: f32 = 10.0;
const MUSHROOM_BASE_ATK: f32 = 1.0;
const MUSHROOM_BASE_MOVE_SPEED: f32 = 100.0;
const MUSHROOM_BASE_ATK_SPEED: f32 = 1.0;
const MUSHROOM_BASE_ATK_RANGE: f32 = 50.0;
const MUSHROOM_BASE_SPORE_COUNT: i32 = 2;

const HERO_EXP_PER_SECOND: f32 = 1.0;

#[derive(Eq, Hash, PartialEq)]
enum ImageType {
    Mushroom,
    Hero,
    MushroomBase,
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
struct AttackTimer {
    value: f32,
}

#[derive(Component)]
struct InCombat {
    value: bool,
}

#[derive(Component)]
struct Mushroom {
    hp: f32,
    atk: f32,
    move_speed: f32,
    atk_speed: f32,
    atk_range: f32,
    spore_count: i32,
}

impl Default for Mushroom {
    fn default() -> Self {
        Mushroom {
            hp: MUSHROOM_BASE_HP,
            atk: MUSHROOM_BASE_ATK,
            move_speed: MUSHROOM_BASE_MOVE_SPEED,
            atk_speed: MUSHROOM_BASE_ATK_SPEED,
            atk_range: MUSHROOM_BASE_ATK_RANGE,
            spore_count: MUSHROOM_BASE_SPORE_COUNT,
        }
    }
}

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Spores {
    count: i32,
}

#[derive(Component)]
struct Hero {
    hp: f32,
    atk: f32,
    move_speed: f32,
    atk_speed: f32,
    atk_range: f32,
    level: i32,
    exp: f32,
    next_level_exp: f32,
}

#[derive(Component)]
struct SporeText;

fn load_assets_system(mut image_manager: ResMut<ImageManager>, asset_server: Res<AssetServer>) {
    let mushroom_sprite_asset: Handle<Image> = asset_server.load("boi.png");
    let mushroom_base_sprite_asset: Handle<Image> = asset_server.load("base.png");
    let ground_sprite_asset: Handle<Image> = asset_server.load("ground.png");
    let hero_sprite_asset: Handle<Image> = asset_server.load("hero.png");

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

    image_manager.images.insert(
        ImageType::Hero,
        Sprite {
            image_handle: hero_sprite_asset,
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
                width: Val::Percent(20.0),
                height: Val::Percent(10.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                left: Val::Percent(1.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Spores:",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 40.0,
                        // Alpha channel of the color controls transparency.
                        color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                    },
                ),
                SporeText,
            ));
        });
}

fn setup_system(
    mut commands: Commands,
    image_manager: Res<ImageManager>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let mushroom_base_sprite = &image_manager[ImageType::MushroomBase];
    let ground_sprite = &image_manager[ImageType::Ground];
    let hero_sprite = &image_manager[ImageType::Hero];

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
                translation: Vec3::new(x_offset - TILE_SIZE, initial_height, 0.0),
                scale: (Vec3::splat(GLOBAL_SCALE)),
                ..default()
            },
            texture: hero_sprite.handle(),
            ..default()
        },
        Hero {
            hp: HERO_BASE_HP,
            atk: HERO_BASE_ATK,
            move_speed: HERO_BASE_MOVE_SPEED,
            atk_speed: HERO_BASE_ATK_SPEED,
            atk_range: HERO_BASE_ATK_RANGE,
            level: HERO_BASE_LEVEL,
            exp: 0.0,
            next_level_exp: HERO_BASE_EXP_REQUIRED,
        },
        AttackTimer { value: 0.0 },
        InCombat { value: false },
    ));

    commands.spawn(Spores {
        count: INITIAL_SPORE_COUNT,
    });

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

fn hero_movement_system(
    mut q_hero: Query<(&mut Hero, &mut Transform, &InCombat)>,
    time: Res<Time>,
) {
    let mut hero = q_hero.single_mut();
    if hero.2.value {
        return;
    }
    hero.1.translation.x -= time.delta_seconds() * hero.0.move_speed;
}

fn hero_level_system(mut q_hero: Query<&mut Hero>, time: Res<Time>) {
    let mut hero = q_hero.single_mut();
    hero.exp += time.delta_seconds() * HERO_EXP_PER_SECOND;

    if hero.exp >= hero.next_level_exp {
        hero.exp = 0.0;
        hero.level += 1;
    }
}

fn hero_attack_system(
    mut q_hero: Query<(&mut Hero, &mut Transform, &mut AttackTimer, &mut InCombat)>,
    mut q_mushroom: Query<(&mut Mushroom, &mut Transform), Without<Hero>>,
) {
    let hero = q_hero.single_mut();
    let mut combat_status = hero.3;
    let mut attack_timer = hero.2;

    combat_status.value = false;
    q_mushroom.for_each_mut(|mushroom| {
        let mushroom_transform = mushroom.1;
        let mut mushroom = mushroom.0;

        let distance = hero.1.translation.x - mushroom_transform.translation.x;
        if distance <= hero.0.atk_range {
            combat_status.value = true;

            if attack_timer.value > 0.0 {
                return;
            }

            mushroom.hp -= hero.0.atk;
            attack_timer.value = 1.0 / hero.0.atk_speed;
        }
    })
}

fn attack_timer_update_system(mut q_attack_timer: Query<&mut AttackTimer>, time: Res<Time>) {
    q_attack_timer.for_each_mut(|mut timer| {
        timer.value -= time.delta_seconds();
    });
}

fn mushroom_death_system(
    mut commands: Commands,
    mut q_mushroom: Query<(Entity, &mut Transform, &mut Mushroom)>,
    mut q_spores: Query<&mut Spores>,
) {
    let mut spores = q_spores.single_mut();
    q_mushroom.for_each_mut(|mushroom| {
        if mushroom.2.hp <= 0.0 {
            commands.entity(mushroom.0).despawn();
            spores.count += mushroom.2.spore_count;
        }
    })
}

fn mushroom_spawn_system(
    mut commands: Commands,
    image_manager: Res<ImageManager>,
    q_mushroom_base: Query<&Transform, With<MushroomBase>>,
    mut q_spores: Query<&mut Spores>,
    mouse: Res<Input<MouseButton>>,
) {
    let mushroom_sprite = &image_manager[ImageType::Mushroom];
    let mushroom_base_position = q_mushroom_base.single().translation;

    let mut spores = q_spores.single_mut();

    if spores.count <= 0 {
        return;
    }

    if mouse.just_pressed(MouseButton::Left) {
        spores.count -= 1;
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
            Mushroom::default(),
            AttackTimer { value: 0.0 },
            InCombat { value: false },
        ));
    }
}

fn mushroom_movement_system(
    mut q_mushroom: Query<(&mut Transform, &Mushroom, &InCombat)>,
    time: Res<Time>,
) {
    q_mushroom.for_each_mut(|mushroom| {
        let mut transform = mushroom.0;
        let mushroom_speed = mushroom.1.move_speed;
        let combat_status = mushroom.2.value;

        if combat_status {
            return;
        }
        transform.translation.x += mushroom_speed * time.delta_seconds();
    });
}

fn mushroom_attack_system(
    mut q_hero: Query<(&mut Hero, &mut Transform)>,
    mut q_mushroom: Query<
        (
            &mut Mushroom,
            &mut Transform,
            &mut AttackTimer,
            &mut InCombat,
        ),
        Without<Hero>,
    >,
) {
    let mut hero = q_hero.single_mut();

    q_mushroom.for_each_mut(|mushroom_data| {
        let mushroom_transform = mushroom_data.1;
        let mushroom = mushroom_data.0;
        let mut attack_timer = mushroom_data.2;
        let mut combat_status = mushroom_data.3;

        let distance = hero.1.translation.x - mushroom_transform.translation.x;
        combat_status.value = false;

        if distance <= mushroom.atk_range {
            combat_status.value = true;

            if attack_timer.value > 0.0 {
                return;
            }

            hero.0.hp -= mushroom.atk;
            attack_timer.value = 1.0 / mushroom.atk_speed;
        }
    });
}

fn spore_text_update_system(
    mut q_spore_text: Query<&mut Text, With<SporeText>>,
    q_spores: Query<&Spores>,
) {
    let mut text = q_spore_text.single_mut();
    let spore_count = q_spores.single().count;

    text.sections[0].value = format!("Spores: {spore_count}");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource::<ImageManager>(ImageManager {
            images: HashMap::new(),
        })
        .add_systems(PreStartup, load_assets_system)
        .add_systems(Startup, (setup_system, setup_ui_system))
        .add_systems(
            Update,
            (
                mushroom_spawn_system,
                mushroom_movement_system,
                mushroom_death_system,
                mushroom_attack_system,
                spore_text_update_system,
                hero_movement_system,
                hero_level_system,
                hero_attack_system,
                attack_timer_update_system,
            ),
        )
        .run();
}
