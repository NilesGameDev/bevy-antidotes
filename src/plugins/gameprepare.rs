use bevy::a11y::accesskit::{NodeBuilder, Role};
use bevy::a11y::AccessibilityNode;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;

use crate::core;
use crate::core::states::GameState;
use crate::core::userinterface::{GAME_THEME_COLOR, NORMAL_BUTTON};
use crate::npc::cell::{CellAttack, CellAttribute, CellBundle};
use crate::npc::goodcell::{GoodCell, GOOD_CELL_SIZE, GOOD_CELL_SPAWN_RADIUS};

use super::antidote::{Substance, SubstanceType, TargetAttribute};
use super::playerresource::PlayerResource;

const BITTER_VALUE_COLOR: Color = Color::hsl(12.0, 1.0, 0.75);
const SWEET_VALUE_COLOR: Color = Color::hsl(160.0, 0.93, 0.74);

pub struct GamePreparePlugin;

impl Plugin for GamePreparePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GamePrepareState>()
            .add_event::<RedrawSubstanceListEvent>()
            .add_event::<InfoMessageEvent>()
            .add_event::<AnimateTestTubeEvent>()
            .add_systems(
                OnEnter(GameState::Prepare),
                (
                    setup_game_prepare,
                    setup_game_prepare_screen,
                    setup_wave_resource,
                ),
            )
            .add_systems(
                OnEnter(GamePrepareState::CreateAntidote),
                setup_create_antidote_screen,
            )
            .add_systems(
                OnEnter(GamePrepareState::CellArrangement),
                setup_cell_arrangement_screen,
            )
            .add_systems(
                Update,
                (
                    game_prepare_btn_action.run_if(in_state(GameState::Prepare)),
                    create_antidote_btn_action.run_if(in_state(GamePrepareState::CreateAntidote)),
                    redraw_substance_list.run_if(in_state(GamePrepareState::CreateAntidote)),
                    add_to_loaded_substances.run_if(in_state(GamePrepareState::CreateAntidote)),
                    mouse_scroll.run_if(in_state(GamePrepareState::CreateAntidote)),
                    display_info_message.run_if(in_state(GamePrepareState::CreateAntidote)),
                    animate_test_tube_fill.run_if(in_state(GamePrepareState::CreateAntidote)),
                    drag_hover_cell_arrangement.run_if(in_state(GamePrepareState::CellArrangement)),
                ),
            )
            .add_systems(
                OnExit(GamePrepareState::CreateAntidote),
                core::despawn_entities::<OnCreateAntidoteScreen>,
            )
            .add_systems(
                OnExit(GamePrepareState::CellArrangement),
                core::despawn_entities::<OnCellArrangementScreen>,
            )
            .add_systems(
                OnExit(GameState::Prepare),
                core::despawn_entities::<OnGamePrepareScreen>,
            );
    }
}

// Game Prepare Events
#[derive(Event, Default)]
struct RedrawSubstanceListEvent;
#[derive(Event, Default)]
struct InfoMessageEvent(String, Color);
#[derive(Event, Default)]
struct AnimateTestTubeEvent;

// Game Prepare Screens
#[derive(Component)]
struct OnGamePrepareScreen;
#[derive(Component)]
struct OnCreateAntidoteScreen;
#[derive(Component)]
struct OnCellArrangementScreen;
#[derive(Component)]
struct CellAttrHoverPanel;

// Game Prepare Core Components
#[derive(Component, Eq, PartialEq)]
pub enum GamePrepareButtonAction {
    CreateAntidote,
    CellArrangement,
}
#[derive(Component)]
enum CreateAntidoteButtonAction {
    ReturnToMainMenu,
    UnloadAll,
    Inject,
    Go,
}
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GamePrepareState {
    CreateAntidote,
    CellArrangement,
    #[default]
    Disabled,
}

// Game Prepare Util Components
#[derive(Component)]
struct GamePrepareSubstanceCard(i32);
#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}
#[derive(Component)]
struct InfoMessageUiHolder;
#[derive(Component)]
struct TestTubeHolder;
#[derive(Component)]
enum CellAttributeHover {
    Health,
    Attack,
    Speed,
    Immune,
    Infection,
}

// Game Prepare Resources
#[derive(Resource, Deref, DerefMut)]
struct DisplayTimer(Timer);

fn setup_game_prepare(mut game_prepare_state: ResMut<NextState<GamePrepareState>>) {
    game_prepare_state.set(GamePrepareState::CreateAntidote);
}

fn setup_game_prepare_screen(mut commands: Commands, player_resources: Res<PlayerResource>) {
    let button_style = Style {
        width: Val::Px(270.0),
        height: Val::Px(40.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::left(Val::Px(1.0)),
        ..default()
    };
    let button_txt_style = TextStyle {
        font_size: 25.0,
        color: GAME_THEME_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnGamePrepareScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(15.0),
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Prepare Lab",
                        TextStyle {
                            font_size: 80.0,
                            color: GAME_THEME_COLOR,
                            ..default()
                        },
                    ));
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(5.0),
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        format!("Wave: {}", player_resources.wave_num),
                        TextStyle {
                            font_size: 40.0,
                            color: GAME_THEME_COLOR,
                            ..default()
                        },
                    ));
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(10.0),
                        align_content: AlignContent::End,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            GamePrepareButtonAction::CreateAntidote,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Create Antidotes",
                                button_txt_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            GamePrepareButtonAction::CellArrangement,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Cell Arrangement",
                                button_txt_style.clone(),
                            ));
                        });
                });
        });
}

fn setup_create_antidote_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut redraw_events: EventWriter<RedrawSubstanceListEvent>,
    game_prepare_screen_query: Query<Entity, With<OnGamePrepareScreen>>,
    mut game_prepare_screen_active_btn_query: Query<(
        &mut BackgroundColor,
        &GamePrepareButtonAction,
    )>,
    current_game_prepare_state: Res<State<GamePrepareState>>,
) {
    // set color of button according to current active screen
    for (mut bg_color, target_btn) in game_prepare_screen_active_btn_query.iter_mut() {
        let current_state = current_game_prepare_state.get();
        if *current_state == GamePrepareState::CreateAntidote
            && *target_btn == GamePrepareButtonAction::CreateAntidote
        {
            *bg_color = Color::hex("#A8786C").unwrap().into();
        } else {
            *bg_color = NORMAL_BUTTON.into();
        }
    }

    let button_style = Style {
        width: Val::Px(360.0),
        height: Val::Px(80.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_txt_style = TextStyle {
        font_size: 32.0,
        color: GAME_THEME_COLOR,
        ..default()
    };
    let test_tube_sprite: Handle<Image> = asset_server.load("sprites/test-tube.png");

    let create_antidote_screen_ent = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(75.0),
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            OnCreateAntidoteScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Percent(80.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(30.0)),
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(70.0),
                                height: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            background_color: Color::hex("#A8786C").unwrap().into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(30.0),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    ..default()
                                },
                                InfoMessageUiHolder,
                            ));

                            let player = AnimationPlayer::default();
                            let anim_tube = Name::new(format!("anim_tube"));

                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Auto,
                                            height: Val::Auto,
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    TestTubeHolder,
                                    player,
                                    anim_tube,
                                ))
                                .with_children(|parent| {
                                    parent.spawn((ImageBundle {
                                        style: Style {
                                            width: Val::Px(400.0),
                                            ..default()
                                        },
                                        image: UiImage::new(test_tube_sprite.clone()),
                                        ..default()
                                    },));
                                });
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                width: Val::Percent(30.0),
                                height: Val::Percent(100.0),
                                align_self: AlignSelf::Stretch,
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    },
                                    ..default()
                                },
                                ScrollingList::default(),
                                AccessibilityNode(NodeBuilder::new(Role::List)),
                            ));
                        });
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Percent(15.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            CreateAntidoteButtonAction::ReturnToMainMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Return",
                                button_txt_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            CreateAntidoteButtonAction::UnloadAll,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Unload All",
                                button_txt_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            CreateAntidoteButtonAction::Inject,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Inject Substances",
                                button_txt_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            CreateAntidoteButtonAction::Go,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Let's Go",
                                button_txt_style.clone(),
                            ));
                        });
                });
        })
        .id();

    let game_prepare_screen_ent = game_prepare_screen_query.single();

    commands
        .entity(game_prepare_screen_ent)
        .add_child(create_antidote_screen_ent);

    redraw_events.send_default();
}

fn setup_wave_resource(mut commands: Commands, mut player_resources: ResMut<PlayerResource>) {
    if player_resources.wave_num == 0 {
        let mut count = 0;
        while count < 2 {
            let substance_id = player_resources.substance_id_gen.0;
            player_resources.substance_collection.insert(
                substance_id,
                Substance {
                    id: substance_id,
                    name: "Initgen".to_string(),
                    target_attribute: TargetAttribute::Immune,
                    value: 50.0,
                    substance_type: SubstanceType::Balanced,
                },
            );
            player_resources.substance_id_gen.0 += 1;
            count += 1;
        }
    }

    commands.insert_resource(DisplayTimer(Timer::from_seconds(4.0, TimerMode::Repeating)));
}

fn setup_cell_arrangement_screen(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut player_resources: ResMut<PlayerResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_prepare_screen_query: Query<Entity, With<OnGamePrepareScreen>>,
    mut game_prepare_screen_active_btn_query: Query<(
        &mut BackgroundColor,
        &GamePrepareButtonAction,
    )>,
    current_game_prepare_state: Res<State<GamePrepareState>>,
) {
    // set color of button according to current active screen
    for (mut bg_color, target_btn) in game_prepare_screen_active_btn_query.iter_mut() {
        let current_state = current_game_prepare_state.get();
        if *current_state == GamePrepareState::CellArrangement
            && *target_btn == GamePrepareButtonAction::CellArrangement
        {
            *bg_color = Color::hex("#A8786C").unwrap().into();
        } else {
            *bg_color = NORMAL_BUTTON.into();
        }
    }

    for (id, good_cell_bundle) in player_resources.cell_army.iter_mut() {
        let mut cell_trans = good_cell_bundle.cell_trans;
        if cell_trans == Vec3::ZERO {
            cell_trans.x =
                rand::thread_rng().gen_range(-GOOD_CELL_SPAWN_RADIUS..=GOOD_CELL_SPAWN_RADIUS);
            cell_trans.y =
                rand::thread_rng().gen_range(-GOOD_CELL_SPAWN_RADIUS..=GOOD_CELL_SPAWN_RADIUS);
        }
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(GOOD_CELL_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_translation(cell_trans),
                ..default()
            },
            GoodCell {
                cell_id: id.clone(),
                cell_size: GOOD_CELL_SIZE,
            },
            OnCellArrangementScreen,
        ));

        good_cell_bundle.cell_trans = cell_trans;
    }

    let sub_attack_img: Handle<Image> = asset_server.load("sprites/sub_attack.png");
    let sub_speed_img: Handle<Image> = asset_server.load("sprites/sub_speed.png");
    let sub_immune_img: Handle<Image> = asset_server.load("sprites/sub_immune.png");
    let sub_health_img: Handle<Image> = asset_server.load("sprites/sub_health.png");
    let sub_infection_img: Handle<Image> = asset_server.load("sprites/sub_infection.png");
    let cell_attr_card_ent = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Auto,
                    height: Val::Auto,
                    justify_content: JustifyContent::Center,
                    align_self: AlignSelf::End,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::right(Val::Px(30.0)),
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            OnCellArrangementScreen,
            CellAttrHoverPanel,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                margin: UiRect::right(Val::Px(10.0)),
                                align_content: AlignContent::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "0",
                                    TextStyle {
                                        font_size: 23.0,
                                        ..default()
                                    },
                                ),
                                CellAttributeHover::Health,
                            ));
                        });
                    parent.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(35.0),
                            height: Val::Px(35.0),
                            ..default()
                        },
                        image: UiImage::new(sub_health_img.clone()),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "Health",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::hex("#fb5b39").unwrap(),
                            ..default()
                        },
                    ));
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                margin: UiRect::right(Val::Px(10.0)),
                                align_content: AlignContent::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "0",
                                    TextStyle {
                                        font_size: 23.0,
                                        ..default()
                                    },
                                ),
                                CellAttributeHover::Attack,
                            ));
                        });
                    parent.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(35.0),
                            height: Val::Px(35.0),
                            ..default()
                        },
                        image: UiImage::new(sub_attack_img.clone()),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "Attack Damage",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::hex("#f8cc3c").unwrap(),
                            ..default()
                        },
                    ));
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                margin: UiRect::right(Val::Px(10.0)),
                                align_content: AlignContent::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "0",
                                    TextStyle {
                                        font_size: 23.0,
                                        ..default()
                                    },
                                ),
                                CellAttributeHover::Speed,
                            ));
                        });
                    parent.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(35.0),
                            height: Val::Px(35.0),
                            ..default()
                        },
                        image: UiImage::new(sub_speed_img.clone()),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "Speed (as seconds)",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::hex("#783eb0").unwrap(),
                            ..default()
                        },
                    ));
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                margin: UiRect::right(Val::Px(10.0)),
                                align_content: AlignContent::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "0",
                                    TextStyle {
                                        font_size: 23.0,
                                        ..default()
                                    },
                                ),
                                CellAttributeHover::Immune,
                            ));
                        });
                    parent.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(35.0),
                            height: Val::Px(35.0),
                            ..default()
                        },
                        image: UiImage::new(sub_immune_img.clone()),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "Immunity",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::hex("#22aaa7").unwrap(),
                            ..default()
                        },
                    ));
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                margin: UiRect::right(Val::Px(10.0)),
                                align_content: AlignContent::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "0",
                                    TextStyle {
                                        font_size: 23.0,
                                        ..default()
                                    },
                                ),
                                CellAttributeHover::Infection,
                            ));
                        });
                    parent.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(35.0),
                            height: Val::Px(35.0),
                            ..default()
                        },
                        image: UiImage::new(sub_infection_img.clone()),
                        ..default()
                    });
                    parent.spawn(TextBundle::from_section(
                        "Infection",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::hex("#fc5d41").unwrap(),
                            ..default()
                        },
                    ));
                });
        })
        .id();

    let game_prepare_screen_ent = game_prepare_screen_query.single();
    commands
        .entity(game_prepare_screen_ent)
        .add_child(cell_attr_card_ent);
}

fn drag_hover_cell_arrangement(
    mouse_buttons: Res<Input<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut cell_attribute_hover_query: Query<(&mut Text, &CellAttributeHover)>,
    mut cell_arrangement_query: Query<(&GoodCell, &mut Transform)>,
    mut gizmos: Gizmos,
    player_resources: ResMut<PlayerResource>,
    mut cell_attr_hover_panel_query: Query<&mut Visibility, With<CellAttrHoverPanel>>,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };
    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    let mut cell_attr_hover_panel = cell_attr_hover_panel_query.single_mut();
    let mut cursor_hit_cell = false;
    for (good_cell, mut cell_trans) in cell_arrangement_query.iter_mut() {
        if Vec2::distance(cell_trans.translation.truncate(), point) <= good_cell.cell_size {
            if mouse_buttons.pressed(MouseButton::Left) {
                cell_trans.translation = point.extend(0.0);
            }

            if let Some(good_cell_attr) = player_resources.cell_army.get(&good_cell.cell_id) {
                for (mut text, cell_attr_hover) in cell_attribute_hover_query.iter_mut() {
                    let text_val = text.sections.first_mut().unwrap();
                    match cell_attr_hover {
                        CellAttributeHover::Health => {
                            text_val.value = format!("{:.2}", good_cell_attr.cell_attribute.health)
                        }
                        CellAttributeHover::Attack => {
                            text_val.value =
                                format!("{:.2}", good_cell_attr.cell_attribute.cell_attack.damage)
                        }
                        CellAttributeHover::Speed => {
                            text_val.value = format!(
                                "{:.2}",
                                good_cell_attr.cell_attribute.cell_attack.attack_rate
                            )
                        }
                        CellAttributeHover::Immune => {
                            text_val.value = format!("{:.2}", good_cell_attr.cell_attribute.immune)
                        }
                        CellAttributeHover::Infection => {
                            text_val.value =
                                format!("{:.2}", good_cell_attr.cell_attribute.infection)
                        }
                    }
                }
            }

            cursor_hit_cell = true;
            *cell_attr_hover_panel = Visibility::Visible;
        }
    }

    if !cursor_hit_cell {
        *cell_attr_hover_panel = Visibility::Hidden;
    }
    gizmos.circle_2d(point, 5.0, Color::WHITE);
}

fn game_prepare_btn_action(
    mut interaction_query: Query<
        (&Interaction, &GamePrepareButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    current_game_prepare_state: Res<State<GamePrepareState>>,
    cell_arrangement_query: Query<(&Transform, &GoodCell)>,
    mut game_prepare_state: ResMut<NextState<GamePrepareState>>,
    mut player_resources: ResMut<PlayerResource>,
) {
    let current_state = current_game_prepare_state.get();
    for (interaction, game_prepare_button_action) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match game_prepare_button_action {
                GamePrepareButtonAction::CreateAntidote => {
                    if *current_state == GamePrepareState::CreateAntidote {
                        continue;
                    }
                    // update the cell arrangement
                    for (cell_trans, good_cell) in cell_arrangement_query.iter() {
                        if let Some(corresponding_cell) =
                            player_resources.cell_army.get_mut(&good_cell.cell_id)
                        {
                            corresponding_cell.cell_trans = cell_trans.translation;
                        }
                    }

                    game_prepare_state.set(GamePrepareState::CreateAntidote);
                }
                GamePrepareButtonAction::CellArrangement => {
                    if *current_state == GamePrepareState::CellArrangement {
                        continue;
                    }
                    game_prepare_state.set(GamePrepareState::CellArrangement);
                }
            }
        }
    }
}

fn create_antidote_btn_action(
    interaction_query: Query<
        (&Interaction, &CreateAntidoteButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut player_resources: ResMut<PlayerResource>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_prepare_state: ResMut<NextState<GamePrepareState>>,
    mut send_info_message_events: EventWriter<InfoMessageEvent>,
    mut redraw_events: EventWriter<RedrawSubstanceListEvent>,
) {
    for (interaction, create_antidote_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match create_antidote_button_action {
                CreateAntidoteButtonAction::ReturnToMainMenu => {
                    game_state.set(GameState::Menu);
                    game_prepare_state.set(GamePrepareState::Disabled);
                }
                CreateAntidoteButtonAction::Go => {
                    if player_resources.cell_army.is_empty() {
                        send_info_message_events.send(InfoMessageEvent(
                            "You have no cells in army. Try to create using Balanced substance!"
                                .to_string(),
                            Color::RED,
                        ));
                        continue;
                    }

                    game_state.set(GameState::Game);
                    game_prepare_state.set(GamePrepareState::Disabled);
                }
                CreateAntidoteButtonAction::UnloadAll => {
                    let temp_loaded_substances = player_resources.loaded_substances.clone();
                    player_resources
                        .substance_collection
                        .extend(temp_loaded_substances);
                    player_resources.loaded_substances.clear();
                    redraw_events.send_default();
                }
                CreateAntidoteButtonAction::Inject => {
                    if player_resources.loaded_substances.is_empty() {
                        send_info_message_events.send(InfoMessageEvent(
                            "No substances put into tube!"
                                .to_string(),
                            Color::RED,
                        ));
                        return;
                    }
                    // now the fun part:
                    // Sweet + Bitter => create 1 good cell
                    // Balanced => create 2 good cells
                    let mut to_spawn_cell_count = 0;
                    let mut total_sweet = 0;
                    let mut total_bitter = 0;
                    let mut total_attack_gain: f32 = 0.0;
                    let mut total_speed_gain: f32 = 0.0;
                    let mut total_immu_gain: f32 = 0.0;
                    let mut total_health_gain: f32 = 0.0;
                    for (_, substance) in player_resources.loaded_substances.iter() {
                        let sweet_factor: f32;
                        match substance.substance_type {
                            SubstanceType::Balanced => {
                                to_spawn_cell_count += 2;
                                sweet_factor = 0.0;
                            }
                            SubstanceType::Bitter => {
                                total_bitter += 1;
                                sweet_factor = -1.0;
                            }
                            SubstanceType::Sweet => {
                                total_sweet += 1;
                                sweet_factor = 1.0;
                            }
                        };

                        match substance.target_attribute {
                            TargetAttribute::Attack => {
                                total_attack_gain += substance.value * sweet_factor
                            }
                            TargetAttribute::Speed => {
                                total_speed_gain -= substance.value * sweet_factor
                            }
                            TargetAttribute::Immune => {
                                total_immu_gain -= substance.value * sweet_factor
                            }
                            TargetAttribute::Health => {
                                total_health_gain += substance.value * sweet_factor
                            }
                        }
                    }

                    to_spawn_cell_count += i32::min(total_bitter, total_sweet);
                    let total_add_infection: f32 = if total_sweet > total_bitter {
                        5.0 * (total_sweet - total_bitter) as f32
                    } else {
                        0.0
                    };

                    // apply modified attribute to all cell
                    for (_, each_cell_bundle) in player_resources.cell_army.iter_mut() {
                        each_cell_bundle.cell_attribute.cell_attack.damage += total_attack_gain;
                        each_cell_bundle.cell_attribute.cell_attack.attack_rate +=
                            f32::min(total_speed_gain, 0.2);
                        each_cell_bundle.cell_attribute.immune += total_immu_gain;
                        each_cell_bundle.cell_attribute.health += total_health_gain;
                        each_cell_bundle.cell_attribute.infection += total_add_infection;
                    }

                    // spawn new cells
                    let mut counter = 0;
                    let mut cell_id = player_resources.good_cell_id.0;
                    while counter < to_spawn_cell_count {
                        player_resources.cell_army.insert(
                            cell_id,
                            CellBundle {
                                cell_trans: Vec3::ZERO,
                                cell_attribute: CellAttribute {
                                    health: 50.0,
                                    cell_attack: CellAttack::new(0.5, 20.0),
                                    immune: 30.0,
                                    infection: 0.0,
                                },
                            },
                        );
                        counter += 1;
                        cell_id += 1;
                    }
                    player_resources.good_cell_id.0 = cell_id;
                    player_resources.loaded_substances.clear();

                    if counter > 0 {
                        send_info_message_events.send(InfoMessageEvent(
                            "New cells created! You can view in Cell Arrangement!".to_string(),
                            GAME_THEME_COLOR,
                        ));
                    }
                }
            }
        }
    }
}

fn add_to_loaded_substances(
    mut interaction_query: Query<
        (&Interaction, &GamePrepareSubstanceCard),
        (Changed<Interaction>, With<Button>),
    >,
    mut player_resources: ResMut<PlayerResource>,
    mut redraw_events: EventWriter<RedrawSubstanceListEvent>,
    mut send_info_message_events: EventWriter<InfoMessageEvent>,
    mut send_animate_test_tube_events: EventWriter<AnimateTestTubeEvent>,
) {
    for (interaction, substance_card) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if player_resources.loaded_substances.len() >= 6 {
                    send_info_message_events.send(InfoMessageEvent(
                        "Can not put more than 6 substances to create an antidote!".to_string(),
                        Color::RED,
                    ));
                    continue;
                }

                if let Some(loaded_substance) = player_resources
                    .substance_collection
                    .remove(&substance_card.0)
                {
                    player_resources
                        .loaded_substances
                        .insert(loaded_substance.id, loaded_substance);
                    redraw_events.send_default();
                }

                send_animate_test_tube_events.send_default();
            }
            _ => (),
        }
    }
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}

fn redraw_substance_list(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut redraw_events: EventReader<RedrawSubstanceListEvent>,
    scrolling_list: Query<(Entity, With<ScrollingList>)>,
    player_resources: Res<PlayerResource>,
) {
    if !redraw_events.is_empty() {
        let sub_attack_img: Handle<Image> = asset_server.load("sprites/sub_attack.png");
        let sub_speed_img: Handle<Image> = asset_server.load("sprites/sub_speed.png");
        let sub_immune_img: Handle<Image> = asset_server.load("sprites/sub_immune.png");
        let sub_health_img: Handle<Image> = asset_server.load("sprites/sub_health.png");
        let mut spawned_card_ents = vec![];

        for (substance_id, substance) in player_resources.substance_collection.iter() {
            let substance_info_img = match substance.target_attribute {
                TargetAttribute::Attack => &sub_attack_img,
                TargetAttribute::Speed => &sub_speed_img,
                TargetAttribute::Health => &sub_health_img,
                TargetAttribute::Immune => &sub_immune_img,
            };

            let spawned_card_ent = commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(80.0),
                            height: Val::Px(100.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        background_color: Color::hex("#5D6965").unwrap().into(),
                        ..default()
                    },
                    AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                ))
                // Inside alignment of each substance card
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // the substance image
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(30.0),
                                        height: Val::Percent(100.0),
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn((
                                        ButtonBundle {
                                            style: Style {
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(100.0),
                                                ..default()
                                            },
                                            image: UiImage::new(substance_info_img.clone()),
                                            background_color: NORMAL_BUTTON.into(),
                                            ..default()
                                        },
                                        GamePrepareSubstanceCard(substance_id.clone()),
                                    ));
                                });

                            // the substance information
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(70.0),
                                        height: Val::Percent(100.0),
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_content: AlignContent::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(20.0),
                                                flex_direction: FlexDirection::Row,
                                                justify_content: JustifyContent::Center,
                                                align_content: AlignContent::Center,
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent
                                                .spawn(NodeBundle {
                                                    style: Style {
                                                        width: Val::Percent(50.0),
                                                        height: Val::Percent(100.0),
                                                        justify_content: JustifyContent::Center,
                                                        align_content: AlignContent::Center,
                                                        ..default()
                                                    },
                                                    ..default()
                                                })
                                                .with_children(|parent| {
                                                    parent.spawn(TextBundle::from_section(
                                                        substance.name.clone(),
                                                        TextStyle {
                                                            color: Color::WHITE.into(),
                                                            font_size: 16.0,
                                                            ..default()
                                                        },
                                                    ));
                                                });
                                            parent
                                                .spawn(NodeBundle {
                                                    style: Style {
                                                        width: Val::Percent(50.0),
                                                        height: Val::Percent(100.0),
                                                        justify_content: JustifyContent::Center,
                                                        align_content: AlignContent::Center,
                                                        ..default()
                                                    },
                                                    ..default()
                                                })
                                                .with_children(|parent| {
                                                    parent.spawn(TextBundle::from_section(
                                                        substance.substance_type.to_string(),
                                                        TextStyle {
                                                            color: Color::hex("#6CA894")
                                                                .unwrap()
                                                                .into(),
                                                            font_size: 16.0,
                                                            ..default()
                                                        },
                                                    ));
                                                });
                                        });
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(80.0),
                                                align_content: AlignContent::Center,
                                                justify_content: JustifyContent::Center,
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            let (value_txt, display_color) =
                                                match substance.substance_type {
                                                    SubstanceType::Bitter => (
                                                        format!(
                                                            "{:.2} {}",
                                                            substance.value,
                                                            substance.target_attribute.to_string()
                                                        ),
                                                        BITTER_VALUE_COLOR,
                                                    ),
                                                    SubstanceType::Sweet => (
                                                        format!(
                                                            "+{:.2} {}",
                                                            substance.value,
                                                            substance.target_attribute.to_string()
                                                        ),
                                                        SWEET_VALUE_COLOR,
                                                    ),
                                                    SubstanceType::Balanced => (
                                                        "Create 2 new cells".to_string(),
                                                        SWEET_VALUE_COLOR,
                                                    ),
                                                };
                                            parent
                                                .spawn(NodeBundle {
                                                    style: Style {
                                                        width: Val::Percent(100.0),
                                                        height: Val::Percent(100.0),
                                                        align_content: AlignContent::Center,
                                                        justify_content: JustifyContent::Center,
                                                        margin: UiRect::top(Val::Px(28.0)),
                                                        ..default()
                                                    },
                                                    ..default()
                                                })
                                                .with_children(|parent| {
                                                    parent.spawn(TextBundle::from_section(
                                                        value_txt,
                                                        TextStyle {
                                                            font_size: 22.0,
                                                            color: display_color,
                                                            ..default()
                                                        },
                                                    ));
                                                });
                                        });
                                });
                        });
                })
                .id();

            spawned_card_ents.push(spawned_card_ent);
        }

        let mut scroll_list_ent = commands.entity(scrolling_list.single().0);
        scroll_list_ent.despawn_descendants();
        scroll_list_ent.push_children(&spawned_card_ents);
        redraw_events.clear();
    }
}

fn display_info_message(
    time: Res<Time>,
    mut commands: Commands,
    mut display_timer: ResMut<DisplayTimer>,
    mut info_message_events: EventReader<InfoMessageEvent>,
    info_place_holder_query: Query<(Entity, With<InfoMessageUiHolder>)>,
) {
    if !info_place_holder_query.is_empty() {
        let mut to_display_message = "".to_string();
        let mut message_color = Color::RED;
        for message_event in info_message_events.read() {
            let message = &message_event.0;
            to_display_message = message.clone();
            message_color = message_event.1;
            break;
        }

        if display_timer.tick(time.delta()).finished() {
            commands
                .entity(info_place_holder_query.single().0)
                .despawn_descendants();
            info_message_events.clear();
        }

        let text_ent = commands
            .spawn(TextBundle::from_section(
                to_display_message,
                TextStyle {
                    font_size: 28.0,
                    color: message_color,
                    ..default()
                },
            ))
            .id();

        let mut placeholder = commands.entity(info_place_holder_query.single().0);
        placeholder.add_child(text_ent);
    }
}

fn animate_test_tube_fill(
    anim_test_tube_events: EventReader<AnimateTestTubeEvent>,
    mut test_tube_query: Query<(&mut AnimationPlayer, &Name, With<TestTubeHolder>)>,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    if anim_test_tube_events.is_empty() {
        return;
    }

    let (mut player, anim_tube, _) = test_tube_query.single_mut();
    let mut animation = AnimationClip::default();
    animation.add_curve_to_path(
        EntityPath {
            parts: vec![anim_tube.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 0.4, 0.8],
            keyframes: Keyframes::Scale(vec![
                Vec3::new(1.0, 1.0, 0.0),
                Vec3::new(1.2, 1.2, 0.0),
                // in case seamless looping is wanted, the last keyframe should
                // be the same as the first one
                Vec3::new(1.0, 1.0, 0.0),
            ]),
        },
    );

    player.play(animations.add(animation));
}
