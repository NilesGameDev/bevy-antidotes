use bevy::prelude::*;

use crate::core;
use crate::core::states::GameState;
use crate::core::userinterface::{GAME_THEME_COLOR, NORMAL_BUTTON};

pub struct GamePreparePlugin;

impl Plugin for GamePreparePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Prepare), setup_game_prepare_screen)
            .add_systems(
                Update,
                game_prepare_btn_action.run_if(in_state(GameState::Prepare)),
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
                                            .spawn(NodeBundle {
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
                                            })
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
                _ => ()
            }
        }
    }
}
