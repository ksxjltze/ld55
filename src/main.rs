use bevy::{prelude::*, utils::HashMap, window::PrimaryWindow};
use rand::Rng;
use std::ops::Index;

const GLOBAL_SCALE: f32 = 1.0;
const TILE_SIZE: f32 = 64.0;
const INITIAL_SPORE_COUNT: i32 = 15;
// const INITIAL_SPORE_COUNT: i32 = 1000;

//HERO
const HERO_BASE_HP: f32 = 1000000.0;
const HERO_BASE_ATK: f32 = 10.0;
const HERO_BASE_MOVE_SPEED: f32 = 10.0;
const HERO_BASE_ATK_SPEED: f32 = 1.0;
const HERO_BASE_ATK_RANGE: f32 = 50.0;
const HERO_BASE_LEVEL: i32 = 1;
const HERO_BASE_EXP_REQUIRED: f32 = 200.0;
const HERO_EXP_PER_SECOND: f32 = 1.0;

//MUSHROOM
const MUSHROOM_BASE_HP: f32 = 10.0;
const MUSHROOM_BASE_ATK: f32 = 0.1;
const MUSHROOM_BASE_MOVE_SPEED: f32 = 100.0;
const MUSHROOM_BASE_ATK_SPEED: f32 = 1.0;
const MUSHROOM_BASE_ATK_RANGE: f32 = 50.0;
const MUSHROOM_BASE_SPORE_COUNT: i32 = 3;
const MUSHROOM_BASE_EXP_DROP: f32 = 1.0;

const MUSHROOM_SPAWN_POSITION_OFFSET_AMOUNT: f32 = 5.0;

//MUSHROOM LORD
const MUSHROOM_LORD_BASE_HP: f32 = 10.0;
const MUSHROOM_LORD_BASE_ATK: f32 = 10.0;
const MUSHROOM_LORD_BASE_MOVE_SPEED: f32 = 100.0;
const MUSHROOM_LORD_BASE_ATK_SPEED: f32 = 1.0;
const MUSHROOM_LORD_BASE_ATK_RANGE: f32 = 100.0;
const MUSHROOM_LORD_BASE_SPORE_COUNT: i32 = 0;
const MUSHROOM_LORD_BASE_EXP_DROP: f32 = 0.0;
const MUSHROOM_LORD_SCALE: f32 = 3.0;

const MUSHROOM_LORD_SPORE_MULTIPLIER_HP: f32 = 0.1;
const MUSHROOM_LORD_SPORE_MULTIPLIER_ATK: f32 = 0.1;
const MUSHROOM_LORD_SPORE_MULTIPLIER_MOVE_SPEED: f32 = 0.0;
const MUSHROOM_LORD_SPORE_MULTIPLIER_ATK_SPEED: f32 = 0.1;
const MUSHROOM_LORD_SPORE_MULTIPLIER_ATK_RANGE: f32 = 0.0;
// const MUSHROOM_LORD_SPORE_MULTIPLIER_SPORE_COUNT: i32 = 0;
// const MUSHROOM_LORD_SPORE_MULTIPLIER_EXP_DROP: f32 = 0.0;

//UI
const NORMAL_BUTTON: Color = Color::rgb(1.0, 1.0, 1.0);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.7, 0.75, 0.5);

//Upgrades
const UPGRADE_SPORE_COUNT_BASE_COST: i32 = 10;
const UPGRADE_MUSHROOMS_PER_CLICK_BASE_COST: i32 = 100;
const UPGRADE_MUSHROOM_HP_BASE_COST: i32 = 500;
const UPGRADE_MUSHROOM_ATK_BASE_COST: i32 = 100;
const UPGRADE_MUSHROOM_ATK_SPEED_BASE_COST: i32 = 10;
const UPGRADE_MUSHROOM_MOVE_SPEED_BASE_COST: i32 = 100;

const UPGRADE_COST_BASE_MULTIPLIER: i32 = 2;
const UPGRADE_COST_SPORE_COUNT_MULTIPLIER: i32 = 3;
const UPGRADE_COST_MUSHROOMS_PER_CLICK_MULTIPLIER: i32 = 3;

//Summoning
const SUMMON_BUTTON_INACTIVE_COLOR: BackgroundColor = BackgroundColor(Color::GRAY);
const SUMMON_BUTTON_ACTIVE_COLOR: BackgroundColor = BackgroundColor(Color::WHITE);
const SUMMON_MINIMUM_SPORE_COUNT: i32 = 1000;

//Etc
const BASE_MUSHROOMS_PER_CLICK: i32 = 1;

#[derive(Eq, Hash, PartialEq)]
enum ImageType {
    Mushroom,
    Hero,
    MushroomBase,
    Ground,
    Background,
    HeroAttack,
}

#[derive(Resource)]
struct ImageManager {
    images: HashMap<ImageType, SpriteImage>,
}

struct SpriteImage {
    image_handle: Handle<Image>,
}

impl SpriteImage {
    fn handle(&self) -> Handle<Image> {
        return self.image_handle.clone_weak();
    }
}

impl ImageManager {
    fn get(&self, key: ImageType) -> &SpriteImage {
        return &self.images[&key];
    }
}

impl Index<ImageType> for ImageManager {
    type Output = SpriteImage;
    fn index(&self, key: ImageType) -> &SpriteImage {
        self.get(key)
    }
}

#[derive(Component)]
struct GameOverText;

#[derive(Component)]
struct GameOverUI;

#[derive(Component)]
struct GameManager {
    game_over: bool,
    victory: bool,
}

impl Default for GameManager {
    fn default() -> Self {
        GameManager {
            game_over: false,
            victory: false,
        }
    }
}

#[derive(Component)]
struct HeroAttackAudio;

#[derive(Component)]
struct GameCamera;

#[derive(Component)]
struct UpgradeButton {
    upgrade_type: UpgradeType,
    cost: i32,
    cost_multiplier: i32,
}
#[derive(Component)]
struct UpgradeButtonText {
    text_type: UpgradeTextType,
}

impl Copy for UpgradeButtonText {}
impl Clone for UpgradeButtonText {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Component)]
struct SummonButton;

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
    xp_drop: f32,
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
            xp_drop: MUSHROOM_BASE_EXP_DROP,
        }
    }
}

impl Copy for Mushroom {}
impl Clone for Mushroom {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Component)]
struct MushroomLord;

#[derive(Eq, Hash, PartialEq)]
enum UpgradeType {
    SporeCount,
    MushroomsPerClick,
    HP,
    ATK,
    MoveSpeed,
    AtkSpeed,
}

#[derive(Eq, Hash, PartialEq)]
enum UpgradeTextType {
    Value,
    Cost,
}

impl Copy for UpgradeTextType {}
impl Clone for UpgradeTextType {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Component)]
struct MushroomManager {
    mushroom_template: Mushroom,
    spawn_count: i32,
}

impl Default for MushroomManager {
    fn default() -> Self {
        MushroomManager {
            mushroom_template: Mushroom::default(),
            spawn_count: BASE_MUSHROOMS_PER_CLICK,
        }
    }
}

#[derive(Component)]
struct SummonManager {
    is_summoned: bool,
}

impl Default for SummonManager {
    fn default() -> Self {
        SummonManager { is_summoned: false }
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

#[derive(Component)]
struct HeroHPText;

#[derive(Component)]
struct HeroEXPText;

#[derive(Component)]
struct HeroLevelText;

#[derive(Component)]
struct MushroomLordHPText;

#[derive(Component)]
struct MushroomLordUI;

fn load_assets_system(mut image_manager: ResMut<ImageManager>, asset_server: Res<AssetServer>) {
    let mushroom_sprite_asset: Handle<Image> = asset_server.load("./boi.png");
    let mushroom_base_sprite_asset: Handle<Image> = asset_server.load("./base.png");
    let ground_sprite_asset: Handle<Image> = asset_server.load("./ground.png");
    let hero_sprite_asset: Handle<Image> = asset_server.load("./hero.png");

    image_manager.images.insert(
        ImageType::Mushroom,
        SpriteImage {
            image_handle: mushroom_sprite_asset,
        },
    );

    image_manager.images.insert(
        ImageType::MushroomBase,
        SpriteImage {
            image_handle: mushroom_base_sprite_asset,
        },
    );

    image_manager.images.insert(
        ImageType::Ground,
        SpriteImage {
            image_handle: ground_sprite_asset,
        },
    );

    image_manager.images.insert(
        ImageType::Hero,
        SpriteImage {
            image_handle: hero_sprite_asset,
        },
    );

    image_manager.images.insert(
        ImageType::HeroAttack,
        SpriteImage {
            image_handle: asset_server.load("./hero_attack.png"),
        },
    );

    image_manager.images.insert(
        ImageType::Background,
        SpriteImage {
            image_handle: asset_server.load("./background.png"),
        },
    );
}

fn setup_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("./fonts/Roboto-Regular.ttf");
    let summoning_circle_image = asset_server.load("./summon_circle.png");

    //Game Over
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK),
                z_index: ZIndex::Global(999),
                visibility: Visibility::Hidden,
                ..default()
            },
            GameOverUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "GAME OVER",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 40.0,
                        // Alpha channel of the color controls transparency.
                        color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                    },
                ),
                GameOverText,
            ));
        });

    //Spores
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(20.0),
                height: Val::Percent(10.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                left: Val::Percent(20.0),
                top: Val::Percent(5.0),
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

    //Mushroom Lord
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(30.0),
                    height: Val::Percent(20.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    top: Val::Percent(10.0),
                    left: Val::Percent(20.0),
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            MushroomLordUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Mushroom Lord HP:",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 20.0,
                        // Alpha channel of the color controls transparency.
                        color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                    },
                ),
                MushroomLordHPText,
            ));
        });

    //Hero stuff
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(30.0),
                height: Val::Percent(20.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                top: Val::Percent(5.0),
                left: Val::Percent(80.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Hero HP:",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 20.0,
                        // Alpha channel of the color controls transparency.
                        color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                    },
                ),
                HeroHPText,
            ));
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Hero EXP:",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 20.0,
                        // Alpha channel of the color controls transparency.
                        color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                    },
                ),
                HeroEXPText,
            ));
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Hero Level:",
                    TextStyle {
                        font: font_handle.clone(),
                        font_size: 20.0,
                        // Alpha channel of the color controls transparency.
                        color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                    },
                ),
                HeroLevelText,
            ));
        });

    let upgrade_button_text_style = TextStyle {
        font: asset_server.load("./fonts/Roboto-Regular.ttf"),
        font_size: 24.0,
        color: Color::BLACK,
    };

    let get_cost_button_bundle = |cost| {
        (
            TextBundle::from_section(format!("Cost: {cost}"), upgrade_button_text_style.clone()),
            UpgradeButtonText {
                text_type: UpgradeTextType::Cost,
            },
        )
    };

    let upgrade_button_font_type = asset_server.load("./fonts/Roboto-Regular.ttf");
    let create_upgrade_button =
        |width, height, upgrade_type, cost, cost_multiplier, text, font_type| {
            return move |parent: &mut ChildBuilder| {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(width),
                                height: Val::Px(height),
                                border: UiRect::all(Val::Px(2.0)),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: BorderColor(Color::BLACK),
                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        },
                        UpgradeButton {
                            upgrade_type: upgrade_type,
                            cost: cost,
                            cost_multiplier: cost_multiplier,
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                text,
                                TextStyle {
                                    font: font_type,
                                    font_size: 24.0,
                                    color: Color::BLACK,
                                },
                            ),
                            UpgradeButtonText {
                                text_type: UpgradeTextType::Value,
                            },
                        ));
                    })
                    .with_children(|parent| {
                        parent.spawn(get_cost_button_bundle(cost));
                    });
            };
        };

    //Upgrades
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect {
                    left: Val::Px(20.0),
                    right: Val::Px(20.0),
                    ..Default::default()
                },
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                left: Val::Percent(0.0),
                top: Val::Percent(20.0),
                ..default()
            },
            ..default()
        })
        //Spores
        .with_children(create_upgrade_button(
            180.0,
            75.0,
            UpgradeType::SporeCount,
            UPGRADE_SPORE_COUNT_BASE_COST,
            UPGRADE_COST_SPORE_COUNT_MULTIPLIER,
            format!("Spore Count: {MUSHROOM_BASE_SPORE_COUNT}"),
            upgrade_button_font_type.clone(),
        ))
        //Mushrooms per click
        .with_children(create_upgrade_button(
            240.0,
            75.0,
            UpgradeType::MushroomsPerClick,
            UPGRADE_MUSHROOMS_PER_CLICK_BASE_COST,
            UPGRADE_COST_MUSHROOMS_PER_CLICK_MULTIPLIER,
            format!("Mushrooms per click: {BASE_MUSHROOMS_PER_CLICK}"),
            upgrade_button_font_type.clone(),
        ))
        .with_children(create_upgrade_button(
            160.0,
            75.0,
            UpgradeType::HP,
            UPGRADE_MUSHROOM_HP_BASE_COST,
            UPGRADE_COST_BASE_MULTIPLIER,
            format!("HP: {MUSHROOM_BASE_HP}"),
            upgrade_button_font_type.clone(),
        ))
        .with_children(create_upgrade_button(
            160.0,
            75.0,
            UpgradeType::ATK,
            UPGRADE_MUSHROOM_ATK_BASE_COST,
            UPGRADE_COST_BASE_MULTIPLIER,
            format!("ATK: {MUSHROOM_BASE_ATK}"),
            upgrade_button_font_type.clone(),
        ))
        .with_children(create_upgrade_button(
            160.0,
            75.0,
            UpgradeType::AtkSpeed,
            UPGRADE_MUSHROOM_ATK_SPEED_BASE_COST,
            UPGRADE_COST_BASE_MULTIPLIER,
            format!("ATK Speed: {MUSHROOM_BASE_ATK_SPEED}"),
            upgrade_button_font_type.clone(),
        ))
        .with_children(create_upgrade_button(
            180.0,
            75.0,
            UpgradeType::MoveSpeed,
            UPGRADE_MUSHROOM_MOVE_SPEED_BASE_COST,
            UPGRADE_COST_BASE_MULTIPLIER,
            format!("Move Speed: {MUSHROOM_BASE_MOVE_SPEED}"),
            upgrade_button_font_type.clone(),
        ));

    //Summon commands
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                top: Val::Percent(37.5),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(128.0),
                        height: Val::Px(128.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    image: UiImage {
                        texture: summoning_circle_image,
                        ..default()
                    },
                    background_color: SUMMON_BUTTON_INACTIVE_COLOR,
                    ..default()
                },
                SummonButton,
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

    //Ground
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

    //Background
    commands.spawn((SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -1.0),
            scale: (Vec3::splat(1.0)),
            ..default()
        },
        texture: image_manager[ImageType::Background].handle(),
        ..default()
    },));

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

    commands.spawn(MushroomManager {
        ..Default::default()
    });

    commands.spawn(SummonManager { ..default() });
    commands.spawn(GameManager::default());
}

fn upgrade_button_system(
    mut q_interaction: Query<
        (&Interaction, &mut UpgradeButton, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut q_mushroom_manager: Query<&mut MushroomManager>,
    mut q_button_text: Query<(&mut Text, &UpgradeButtonText)>,
    mut q_spores: Query<&mut Spores>,
    q_hero: Query<&Hero>,
) {
    let mut manager = q_mushroom_manager.single_mut();
    let mut spores = q_spores.single_mut();
    let hero = q_hero.single();

    for (interaction, mut button, children) in &mut q_interaction {
        match *interaction {
            Interaction::Pressed => {
                if spores.count < button.cost {
                    continue;
                }

                spores.count -= button.cost;
                button.cost *= button.cost_multiplier;
                let cost = button.cost;

                let mut update_button_text = |child, text: &String| {
                    match q_button_text.get_mut(child) {
                        Ok((mut button_text, upgrade_button_text)) => {
                            match upgrade_button_text.text_type {
                                UpgradeTextType::Value => {
                                    button_text.sections[0].value = text.clone();
                                }
                                UpgradeTextType::Cost => {
                                    button_text.sections[0].value = format!("Cost: {cost}")
                                }
                            }
                        }
                        Err(_) => todo!(),
                    };
                };

                let mut update_button_children = |text: String| {
                    for &child in children.iter() {
                        update_button_text(child, &text);
                    }
                };

                match button.upgrade_type {
                    UpgradeType::SporeCount => {
                        manager.mushroom_template.spore_count += hero.level;
                        let spore_count = manager.mushroom_template.spore_count;

                        update_button_children(format!("Spore count: {spore_count}"));
                    }
                    UpgradeType::MushroomsPerClick => {
                        manager.spawn_count += 1;
                        let spawn_count = manager.spawn_count;

                        update_button_children(format!("Mushrooms per click: {spawn_count}"));
                    }
                    UpgradeType::HP => {
                        manager.mushroom_template.hp += MUSHROOM_BASE_HP;
                        let hp = manager.mushroom_template.hp;

                        update_button_children(format!("HP: {hp}"));
                    }
                    UpgradeType::ATK => {
                        manager.mushroom_template.atk += MUSHROOM_BASE_ATK;
                        let atk = manager.mushroom_template.atk;

                        update_button_children(format!("ATK: {atk}"));
                    }
                    UpgradeType::AtkSpeed => {
                        manager.mushroom_template.atk_speed += MUSHROOM_BASE_ATK_SPEED;
                        let atk_speed = manager.mushroom_template.atk_speed;

                        update_button_children(format!("ATK Speed: {atk_speed}"));
                    }
                    UpgradeType::MoveSpeed => {
                        manager.mushroom_template.move_speed += MUSHROOM_BASE_MOVE_SPEED;
                        let move_speed = manager.mushroom_template.move_speed;

                        update_button_children(format!("Move Speed: {move_speed}"));
                    }
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color, _children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn game_over_system(
    q_game_manager: Query<&GameManager>,
    mut q_game_over_ui: Query<&mut Visibility, With<GameOverUI>>,
    mut q_game_over_text: Query<&mut Text, With<GameOverText>>,
) {
    let game_manager = q_game_manager.single();
    let mut visibility = q_game_over_ui.single_mut();
    let mut game_over_text = q_game_over_text.single_mut();

    if game_manager.game_over {
        *visibility = Visibility::Visible;

        if game_manager.victory {
            game_over_text.sections[0].value = "VICTORY".to_string();
        } else {
            game_over_text.sections[0].value = "GAME OVER".to_string();
        }
    }
}

fn summon_button_system(
    mut commands: Commands,
    mut q_summon_button_interaction: Query<
        (&Interaction, &mut BackgroundColor),
        With<SummonButton>,
    >,
    image_manager: Res<ImageManager>,
    q_mushroom_base: Query<&Transform, With<MushroomBase>>,
    mut q_summon_manager: Query<&mut SummonManager>,
    mut q_spores: Query<&mut Spores>,
    mut q_mushroom_lord_ui_visibility: Query<&mut Visibility, With<MushroomLordUI>>,
    mut q_mushroom_lord_hp_text: Query<&mut Text, With<MushroomLordHPText>>,
) {
    let mushroom_sprite = &image_manager[ImageType::Mushroom];
    let mushroom_base_position = q_mushroom_base.single().translation;
    let mut summon_manager = q_summon_manager.single_mut();
    let mut spores = q_spores.single_mut();

    let mut mushroom_lord_hp_text = q_mushroom_lord_hp_text.single_mut();
    let mut mushroom_lord_ui_visibility = q_mushroom_lord_ui_visibility.single_mut();

    for (interaction, mut background_color) in &mut q_summon_button_interaction {
        if spores.count < SUMMON_MINIMUM_SPORE_COUNT {
            return;
        } else {
            *background_color = SUMMON_BUTTON_ACTIVE_COLOR;
        }

        match *interaction {
            Interaction::Pressed => {
                if summon_manager.is_summoned {
                    return;
                }

                let mushroom_lord_stats = Mushroom {
                    hp: MUSHROOM_LORD_BASE_HP
                        + MUSHROOM_LORD_BASE_HP
                            * MUSHROOM_LORD_SPORE_MULTIPLIER_HP
                            * spores.count as f32,
                    atk: MUSHROOM_LORD_BASE_ATK
                        + MUSHROOM_LORD_BASE_ATK
                            * MUSHROOM_LORD_SPORE_MULTIPLIER_ATK
                            * spores.count as f32,
                    move_speed: MUSHROOM_LORD_BASE_MOVE_SPEED
                        + MUSHROOM_LORD_BASE_MOVE_SPEED
                            * MUSHROOM_LORD_SPORE_MULTIPLIER_MOVE_SPEED
                            * spores.count as f32,
                    atk_speed: MUSHROOM_LORD_BASE_ATK_SPEED
                        + MUSHROOM_LORD_BASE_ATK_SPEED
                            * MUSHROOM_LORD_SPORE_MULTIPLIER_ATK_SPEED
                            * spores.count as f32,
                    atk_range: MUSHROOM_LORD_BASE_ATK_RANGE
                        + MUSHROOM_LORD_BASE_ATK_RANGE
                            * MUSHROOM_LORD_SPORE_MULTIPLIER_ATK_RANGE
                            * spores.count as f32,
                    spore_count: MUSHROOM_LORD_BASE_SPORE_COUNT,
                    xp_drop: MUSHROOM_LORD_BASE_EXP_DROP,
                };

                let mushroom_lord_hp = mushroom_lord_stats.hp;

                commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(
                                mushroom_base_position.x + MUSHROOM_SPAWN_POSITION_OFFSET_AMOUNT,
                                mushroom_base_position.y + 32.0 * (MUSHROOM_LORD_SCALE - 1.0),
                                1.0,
                            ),
                            scale: (Vec3::splat(MUSHROOM_LORD_SCALE)),
                            ..default()
                        },
                        texture: mushroom_sprite.handle(),
                        ..default()
                    },
                    mushroom_lord_stats,
                    MushroomLord,
                    AttackTimer { value: 0.0 },
                    InCombat { value: false },
                ));
                spores.count = 0;
                summon_manager.is_summoned = true;

                *mushroom_lord_ui_visibility = Visibility::Visible;
                mushroom_lord_hp_text.sections[0].value =
                    format!("Mushroom Lord HP: {mushroom_lord_hp}");
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn hero_movement_system(
    mut q_hero: Query<(&mut Hero, &mut Transform, &InCombat)>,
    time: Res<Time>,
) {
    let (hero, mut hero_transform, combat_status) = q_hero.single_mut();
    if combat_status.value {
        return;
    }
    hero_transform.translation.x -= time.delta_seconds() * hero.move_speed;
}

fn hero_level_system(mut q_hero: Query<&mut Hero>, time: Res<Time>) {
    let mut hero = q_hero.single_mut();
    hero.exp += time.delta_seconds() * HERO_EXP_PER_SECOND;

    if hero.exp >= hero.next_level_exp {
        hero.exp = 0.0;
        hero.level += 1;

        hero.hp = hero.level as f32 * HERO_BASE_HP;
        hero.atk = hero.level as f32 * HERO_BASE_ATK;
        hero.move_speed = hero.level as f32 * HERO_BASE_MOVE_SPEED;
        hero.atk_speed = hero.level as f32 * HERO_BASE_ATK_SPEED;
        hero.atk_range = hero.level as f32 * HERO_BASE_ATK_RANGE;
        hero.next_level_exp = hero.level as f32 * HERO_BASE_EXP_REQUIRED;
    }
}

//lol
fn hero_attack_system(
    mut q_hero: Query<(&mut Hero, &mut Transform, &mut AttackTimer, &mut InCombat)>,
    mut q_mushroom: Query<(&mut Mushroom, &mut Transform), Without<Hero>>,
    mut q_hero_sprite: Query<(&mut Sprite, &mut Handle<Image>), With<Hero>>,
    q_mushroom_base: Query<&Transform, (With<MushroomBase>, Without<Hero>, Without<Mushroom>)>,
    mut q_game_manager: Query<&mut GameManager>,
    image_manager: ResMut<ImageManager>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let (hero, hero_transform, mut hero_attack_timer, mut hero_combat_status) = q_hero.single_mut();
    let mushroom_base = q_mushroom_base.single();
    let mut game_manager = q_game_manager.single_mut();

    if game_manager.game_over {
        hero_combat_status.value = false;
        return;
    }

    let distance_to_base = hero_transform.translation.x - mushroom_base.translation.x;
    if distance_to_base <= 1.0 {
        game_manager.game_over = true;
        game_manager.victory = false;
    }

    let (mut sprite, mut texture) = q_hero_sprite.single_mut();

    if hero_attack_timer.value <= 0.0 {
        hero_combat_status.value = false;
    }

    q_mushroom.for_each_mut(|mushroom| {
        let mushroom_transform = mushroom.1;
        let mut mushroom = mushroom.0;

        let distance = (hero_transform.translation.x - mushroom_transform.translation.x).abs();
        if distance <= hero.atk_range {
            hero_combat_status.value = true;
            if hero_attack_timer.value > 0.0 {
                return;
            }

            mushroom.hp -= hero.atk;
        }
    });

    let cooldown = 1.0 / hero.atk_speed;
    if hero_attack_timer.value <= 0.0 {
        if hero_combat_status.value {
            *texture = image_manager[ImageType::HeroAttack].handle();

            //hack to make it play every time
            //lazy to make another system for this
            commands.spawn((
                AudioBundle {
                    source: asset_server.load("./ough.ogg"),
                    settings: PlaybackSettings::DESPAWN,
                    ..default()
                },
                HeroAttackAudio,
            ));
        }
        hero_attack_timer.value = cooldown;
    } else if hero_attack_timer.value <= (cooldown / 2.0) {
        *texture = image_manager[ImageType::Hero].handle();
    }

    if hero_combat_status.value {
        sprite.color.set_r(0.0);
        sprite.color.set_g(1.0);
        sprite.color.set_b(1.0);
    } else {
        sprite.color.set_r(1.0);
        sprite.color.set_g(1.0);
        sprite.color.set_b(1.0);
    }
}

fn attack_timer_update_system(mut q_attack_timer: Query<&mut AttackTimer>, time: Res<Time>) {
    q_attack_timer.for_each_mut(|mut timer| {
        timer.value -= time.delta_seconds();
    });
}

fn mushroom_death_system(
    mut commands: Commands,
    mut q_mushroom: Query<(Entity, &mut Transform, &mut Mushroom)>,
    mut q_mushroom_manager: Query<&mut MushroomManager>,
    mut q_spores: Query<&mut Spores>,
    mut q_hero: Query<&mut Hero>,
) {
    let mut spores = q_spores.single_mut();
    let mut hero = q_hero.single_mut();
    let mushroom_manager = q_mushroom_manager.single_mut();

    q_mushroom.for_each_mut(|mushroom| {
        if mushroom.2.hp <= 0.0 {
            commands.entity(mushroom.0).despawn();
            spores.count += mushroom_manager.mushroom_template.spore_count;
            hero.exp += mushroom.2.xp_drop;
        }
    })
}

fn hero_death_system(mut q_hero: Query<&Hero>, mut q_game_manager: Query<&mut GameManager>) {
    let hero = q_hero.single_mut();
    let mut game_manager = q_game_manager.single_mut();

    if hero.hp <= 0.0 {
        game_manager.game_over = true;
        game_manager.victory = true;
    }
}

fn mushroom_spawn_system(
    mut commands: Commands,
    image_manager: Res<ImageManager>,
    q_mushroom_base: Query<&Transform, With<MushroomBase>>,
    mut q_spores: Query<&mut Spores>,
    q_mushroom_manager: Query<&MushroomManager>,
    mouse: Res<Input<MouseButton>>,
) {
    let mushroom_sprite = &image_manager[ImageType::Mushroom];
    let mushroom_base_position = q_mushroom_base.single().translation;

    let mut spores = q_spores.single_mut();
    let mushroom_manager = q_mushroom_manager.single();

    if spores.count <= 0 {
        return;
    }

    if mouse.just_pressed(MouseButton::Left) {
        let mut rng = rand::thread_rng();
        let mut spawn_count = mushroom_manager.spawn_count;

        if spores.count - spawn_count < 0 {
            spawn_count = spores.count;
        }

        for _i in 0..spawn_count {
            let random_offset: f32 = rng.gen();

            spores.count -= 1;
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            mushroom_base_position.x
                                + random_offset * MUSHROOM_SPAWN_POSITION_OFFSET_AMOUNT,
                            mushroom_base_position.y,
                            0.0,
                        ),
                        scale: (Vec3::splat(GLOBAL_SCALE)),
                        ..default()
                    },
                    texture: mushroom_sprite.handle(),
                    ..default()
                },
                mushroom_manager.mushroom_template,
                AttackTimer { value: 0.0 },
                InCombat { value: false },
            ));
        }
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
            &mut Sprite,
        ),
        Without<Hero>,
    >,
) {
    let mut hero = q_hero.single_mut();

    q_mushroom.for_each_mut(|mushroom_data| {
        let mut mushroom_transform = mushroom_data.1;
        let mushroom = mushroom_data.0;
        let mut attack_timer = mushroom_data.2;
        let mut combat_status = mushroom_data.3;

        let mut sprite = mushroom_data.4;

        let distance = hero.1.translation.x - mushroom_transform.translation.x;
        combat_status.value = false;

        if distance <= mushroom.atk_range {
            combat_status.value = true;

            let cooldown = 1.0 / mushroom.atk_speed;
            if attack_timer.value <= (cooldown * 0.5) {
                mushroom_transform.scale.y = 1.0;
            }

            if attack_timer.value > 0.0 {
                return;
            }

            hero.0.hp -= mushroom.atk;

            if combat_status.value {
                mushroom_transform.scale.y = 1.1;
            }
            attack_timer.value = cooldown;
        }

        if combat_status.value {
            sprite.color.set_r(0.0);
            sprite.color.set_g(1.0);
            sprite.color.set_b(1.0);
        } else {
            sprite.color.set_r(1.0);
            sprite.color.set_g(1.0);
            sprite.color.set_b(1.0);
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

fn mushroom_lord_ui_update_system(
    mut q_mushroom_lord_hp_text: Query<&mut Text, With<MushroomLordHPText>>,
    q_mushroom_lord: Query<&Mushroom, With<MushroomLord>>,
) {
    let q_mushroom_lord_result = q_mushroom_lord.get_single();
    match q_mushroom_lord_result {
        Ok(mushroom_lord) => {
            let mut mushroom_lord_hp_text = q_mushroom_lord_hp_text.single_mut();
            let hp = mushroom_lord.hp;

            mushroom_lord_hp_text.sections[0].value = format!("Mushroom Lord HP: {hp}");
        }
        Err(_) => (),
    }
}

fn hero_hp_text_update_system(
    mut q_hero_hp_text: Query<&mut Text, With<HeroHPText>>,
    q_hero: Query<&Hero>,
) {
    let mut text = q_hero_hp_text.single_mut();
    let hero_hp = q_hero.single().hp;

    text.sections[0].value = format!("Hero HP: {hero_hp}");
}

fn hero_exp_text_update_system(
    mut q_hero_exp_text: Query<&mut Text, With<HeroEXPText>>,
    q_hero: Query<&Hero>,
) {
    let mut text = q_hero_exp_text.single_mut();
    let hero_exp = q_hero.single().exp;

    text.sections[0].value = format!("Hero EXP: {hero_exp}");
}

fn hero_level_text_update_system(
    mut q_hero_level_text: Query<&mut Text, With<HeroLevelText>>,
    q_hero: Query<&Hero>,
) {
    let mut text = q_hero_level_text.single_mut();
    let hero_level = q_hero.single().level;

    text.sections[0].value = format!("Hero Level: {hero_level}");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource::<ImageManager>(ImageManager {
            images: HashMap::new(),
        })
        .add_systems(PreStartup, load_assets_system)
        .add_systems(Startup, (setup_system, setup_ui_system))
        .add_systems(PreUpdate, (attack_timer_update_system, hero_attack_system))
        .add_systems(
            Update,
            (
                //MUSHROOM
                mushroom_spawn_system,
                mushroom_movement_system,
                mushroom_death_system,
                mushroom_attack_system,
                mushroom_lord_ui_update_system,
                spore_text_update_system,
                //HERO
                hero_hp_text_update_system,
                hero_exp_text_update_system,
                hero_level_text_update_system,
                hero_movement_system,
                hero_level_system,
                hero_death_system,
                //UI
                button_system,
                upgrade_button_system,
                summon_button_system,
                game_over_system,
            ),
        )
        .run();
}
