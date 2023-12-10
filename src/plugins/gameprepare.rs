use bevy::a11y::accesskit::{NodeBuilder, Role};
use bevy::a11y::AccessibilityNode;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use crate::core;
use crate::core::states::GameState;
use crate::core::userinterface::{GAME_THEME_COLOR, NORMAL_BUTTON};
use crate::npc::cell::{CellAttack, CellAttribute};

use super::antidote::{Substance, SubstanceType, TargetAttribute};
use super::playerresource::PlayerResource;

const BITTER_VALUE_COLOR: Color = Color::hsl(12.0, 1.0, 0.75);
const SWEET_VALUE_COLOR: Color = Color::hsl(160.0, 0.93, 0.74);

pub struct GamePreparePlugin;

impl Plugin for GamePreparePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RedrawSubstanceListEvent>()
            .add_event::<InfoMessageEvent>()
            .add_event::<AnimateTestTubeEvent>()
            .add_systems(
                OnEnter(GameState::Prepare),
                (setup_game_prepare_screen, setup_wave_resource),
            )
            .add_systems(
                Update,
                (
                    redraw_substance_list.run_if(in_state(GameState::Prepare)),
                    game_prepare_btn_action.run_if(in_state(GameState::Prepare)),
                    add_to_loaded_substances.run_if(in_state(GameState::Prepare)),
                    mouse_scroll.run_if(in_state(GameState::Prepare)),
                    display_info_message.run_if(in_state(GameState::Prepare)),
                    animate_test_tube_fill.run_if(in_state(GameState::Prepare)),
                ),
            )
            .add_systems(
                OnExit(GameState::Prepare),
                core::despawn_entities::<OnGamePrepareScreen>,
            );
    }
}

#[derive(Event, Default)]
struct RedrawSubstanceListEvent;
#[derive(Event, Default)]
struct InfoMessageEvent(String);
#[derive(Event, Default)]
struct AnimateTestTubeEvent;

#[derive(Component)]
struct OnGamePrepareScreen;

#[derive(Component)]
enum GamePrepareButtonAction {
    ReturnToMainMenu,
    Inject,
    Go,
}

#[derive(Component)]
struct GamePrepareSubstanceCard(i32);

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

#[derive(Component)]
struct InfoMessageUiHolder;

#[derive(Resource, Deref, DerefMut)]
struct DisplayTimer(Timer);

#[derive(Component)]
struct TestTubeHolder;

fn setup_game_prepare_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut redraw_events: EventWriter<RedrawSubstanceListEvent>,
) {
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
                        height: Val::Percent(90.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
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
                                margin: UiRect::all(Val::Px(20.0)),
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
                                        height: Val::Percent(100.0),
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
                                            width: Val::Px(450.0),
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
                            GamePrepareButtonAction::ReturnToMainMenu,
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
                            GamePrepareButtonAction::Inject,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Inject",
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
                            GamePrepareButtonAction::Go,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Let's Go",
                                button_txt_style.clone(),
                            ));
                        });
                });
        });

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

fn game_prepare_btn_action(
    interaction_query: Query<
        (&Interaction, &GamePrepareButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut player_resources: ResMut<PlayerResource>,
    mut game_state: ResMut<NextState<GameState>>,
    mut send_info_message_events: EventWriter<InfoMessageEvent>,
) {
    for (interaction, game_prepare_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match game_prepare_button_action {
                GamePrepareButtonAction::ReturnToMainMenu => {
                    game_state.set(GameState::Menu);
                }
                GamePrepareButtonAction::Go => {
                    if player_resources.cell_army.is_empty() {
                        send_info_message_events.send(InfoMessageEvent(
                            "You have no cells in army. Try to create using Balanced substance!"
                                .to_string(),
                        ));
                        continue;
                    }
                    game_state.set(GameState::Game)
                }
                GamePrepareButtonAction::Inject => {
                    if player_resources.loaded_substances.is_empty() {
                        continue;
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
                    for (_, each_cell) in player_resources.cell_army.iter_mut() {
                        each_cell.cell_attack.damage += total_attack_gain;
                        each_cell.cell_attack.attack_rate += f32::min(total_speed_gain, 0.2);
                        each_cell.immune += total_immu_gain;
                        each_cell.health += total_health_gain;
                        each_cell.infection += total_add_infection;
                    }

                    let mut counter = 0;
                    let mut cell_id = player_resources.good_cell_id.0;
                    while counter < to_spawn_cell_count {
                        player_resources.cell_army.insert(
                            cell_id,
                            CellAttribute {
                                health: 50.0,
                                cell_attack: CellAttack::new(0.5, 20.0),
                                immune: 30.0,
                                infection: 0.0,
                            },
                        );
                        counter += 1;
                        cell_id += 1;
                    }
                    player_resources.good_cell_id.0 = cell_id;
                    player_resources.loaded_substances.clear();

                    if counter > 0 {
                        send_info_message_events.send(InfoMessageEvent(
                            "New cells created!".to_string(),
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
                if player_resources.loaded_substances.len() > 4 {
                    send_info_message_events.send(InfoMessageEvent(
                        "Can not put more than 4 substances to create an antidote!".to_string(),
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
        for message_event in info_message_events.read() {
            let message = &message_event.0;
            to_display_message = message.clone();
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
                    color: Color::RED,
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
