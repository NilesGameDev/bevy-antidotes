use bevy::prelude::*;

use crate::core;
use crate::core::states::GameState;
use crate::core::userinterface::{GAME_THEME_COLOR, NORMAL_BUTTON};
use crate::npc::cell::{CellAttribute, CellAttack};

use super::antidote::{SubstanceType, TargetAttribute};
use super::playerresource::PlayerResource;

pub struct GamePreparePlugin;

impl Plugin for GamePreparePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Prepare), setup_game_prepare_screen)
            .add_systems(
                Update,
                (game_prepare_btn_action.run_if(in_state(GameState::Prepare)),
                        add_to_loaded_substances.run_if(in_state(GameState::Prepare))),
            )
            .add_systems(
                OnExit(GameState::Prepare),
                core::despawn_entities::<OnGamePrepareScreen>,
            );
    }
}

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

fn setup_game_prepare_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                                ..default()
                            },
                            background_color: Color::hex("#A8786C").unwrap().into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(450.0),
                                    ..default()
                                },
                                image: UiImage::new(test_tube_sprite.clone()),
                                ..default()
                            });
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(30.0),
                                height: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    let mut count = 0;
                                    while count < 6 {
                                        parent
                                            .spawn((NodeBundle {
                                                style: Style {
                                                    width: Val::Percent(90.0),
                                                    height: Val::Px(100.0),
                                                    margin: UiRect::all(Val::Px(10.0)),
                                                    ..default()
                                                },
                                                background_color: Color::hex("#5D6965")
                                                    .unwrap()
                                                    .into(),
                                                border_color: Color::RED.into(),
                                                ..default()
                                            },
                                            GamePrepareSubstanceCard(count))
                                        )
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
                                                                    width: Val::Percent(10.0),
                                                                    height: Val::Percent(100.0),
                                                                    justify_content: JustifyContent::Center,
                                                                    ..default()
                                                                },
                                                                background_color: Color::PURPLE.into(),
                                                                ..default()
                                                            })
                                                            .with_children(|parent| {
                                                                parent.spawn(ImageBundle {
                                                                    image: UiImage::new(
                                                                        test_tube_sprite.clone(),
                                                                    ),
                                                                    ..default()
                                                                });
                                                            });
                                                        
                                                        // the substance information
                                                        parent.spawn(NodeBundle {
                                                            style: Style {
                                                                width: Val::Percent(90.0),
                                                                height: Val::Percent(100.0),
                                                                flex_direction: FlexDirection::Column,
                                                                margin: UiRect::left(Val::Px(30.0)),
                                                                ..default()
                                                            },
                                                            ..default()
                                                        })
                                                        .with_children(|parent| {
                                                            parent.spawn(NodeBundle {
                                                                style: Style {
                                                                    width: Val::Percent(20.0),
                                                                    height: Val::Percent(100.0),
                                                                    flex_direction: FlexDirection::Row,
                                                                    ..default()
                                                                },
                                                                ..default()
                                                            })
                                                            .with_children(|parent| {
                                                                parent.spawn(TextBundle::from_section("Atagen", TextStyle {
                                                                    color: Color::WHITE.into(),
                                                                    ..default()
                                                                }));
                                                                parent.spawn(TextBundle::from_section("Sweet Substance", TextStyle {
                                                                    color: Color::hex("#6CA894").unwrap().into(),
                                                                    ..default()
                                                                }));
                                                            });
                                                            parent.spawn(NodeBundle {
                                                                style: Style {
                                                                    width: Val::Percent(80.0),
                                                                    height: Val::Percent(100.0),
                                                                    align_content: AlignContent::Center,
                                                                    justify_content: JustifyContent::Center,
                                                                    ..default()
                                                                },
                                                                ..default()
                                                            })
                                                            .with_children(|parent| {
                                                                parent.spawn(TextBundle::from_section("↑ 20.3 Attack", TextStyle {
                                                                    font_size: 32.0, 
                                                                    color: Color::GREEN.into(),
                                                                    ..default()
                                                                }));
                                                            });
                                                        });
                                                    });
                                            });
                                        count += 1;
                                    }
                                });
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
}

fn game_prepare_btn_action(
    interaction_query: Query<
        (&Interaction, &GamePrepareButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut player_resources: ResMut<PlayerResource>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, game_prepare_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match game_prepare_button_action {
                GamePrepareButtonAction::ReturnToMainMenu => {
                    game_state.set(GameState::Menu);
                }
                GamePrepareButtonAction::Go => {
                    game_state.set(GameState::Game)
                }
                GamePrepareButtonAction::Inject => {
                    // now the fun part:
                    // Sweet + Bitter => create 1 good cell
                    // Balanced => create 2 good cells
                    let mut to_spawn_cell_count = 0;
                    let mut total_sweet = 0;
                    let mut total_bitter = 0;
                    let mut sweet_factor = 1.0;
                    let mut total_attack_gain: f32 = 0.0;
                    let mut total_speed_gain: f32 = 0.0;
                    let mut total_immu_gain: f32 = 0.0;
                    let mut total_health_gain: f32 = 0.0;
                    for (_, substance) in player_resources.loaded_substances.iter() {
                        (to_spawn_cell_count, total_sweet, total_bitter, sweet_factor) = match substance.substance_type  {
                            SubstanceType::Balanced => {
                                (to_spawn_cell_count + 2, 0 ,0, 0.0)
                            }
                            SubstanceType::Bitter => (0, 0, total_sweet + 1, -1.0),
                            SubstanceType::Sweet => (0, total_bitter + 1, 0, 1.0),
                        };

                        match substance.target_attribute {
                            TargetAttribute::Attack => total_attack_gain += substance.value * sweet_factor,
                            TargetAttribute::Speed => total_speed_gain -= substance.value * sweet_factor,
                            TargetAttribute::Immune => total_immu_gain += substance.value * sweet_factor,
                            TargetAttribute::Health => total_health_gain += substance.value * sweet_factor
                        }
                    }

                    to_spawn_cell_count += i32::min(total_bitter, total_sweet);
                    let total_add_infection: f32 =  if total_sweet > total_bitter {
                        5.0 * (total_sweet - total_bitter) as f32
                    } else {
                        0.0
                    };

                    // apply modified attribute to all cell
                    for (_, each_cell) in player_resources.cell_army.iter_mut() {
                        each_cell.cell_attack.damage += total_attack_gain;
                        each_cell.cell_attack.attack_rate += total_speed_gain;
                        each_cell.immune += total_immu_gain;
                        each_cell.health += total_health_gain;
                        each_cell.infection += total_add_infection;
                    }

                    let mut counter = 0;
                    let mut cell_id = player_resources.good_cell_id.0;
                    while counter < to_spawn_cell_count {
                        player_resources.cell_army.insert(
                            cell_id,CellAttribute {
                            health: 50.0,
                            cell_attack: CellAttack::new(2.0, 20.0),
                            immune: 30.0,
                            infection: 0.0
                        });
                        counter += 1;
                        cell_id += 1;
                    }
                    player_resources.good_cell_id.0 = cell_id;
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
    mut player_resources: ResMut<PlayerResource>
) {
    for (interaction, substance_card) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(loaded_substance) = player_resources.substance_collection.remove(&substance_card.0) {
                    player_resources.loaded_substances.insert(loaded_substance.id, loaded_substance);
                }
            },
            _ => ()
        }
    }
}
