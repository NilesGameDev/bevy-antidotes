use bevy::prelude::*;

use crate::core;
use crate::core::states::GameState;
use crate::core::userinterface::GAME_THEME_COLOR;

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
    // let button_style = Style {
    //     width: Val::Px(360.0),
    //     height: Val::Px(80.0),
    //     margin: UiRect::all(Val::Px(20.0)),
    //     justify_content: JustifyContent::Center,
    //     align_items: AlignItems::Center,
    //     ..default()
    // };

    // let button_txt_style = TextStyle {
    //     font_size: 32.0,
    //     color: GAME_THEME_COLOR,
    //     ..default()
    // };

    let test_tube_sprite: Handle<Image> = asset_server.load("sprites/test-tube.png");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
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
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::hex("#6E4A41").unwrap().into(),
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
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                image: UiImage::new(test_tube_sprite),
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
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    let mut count = 0;
                                    while count < 5 {
                                        parent.spawn(NodeBundle {
                                            style: Style {
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Center,
                                                margin: UiRect::all(Val::Px(10.0)),
                                                ..default()
                                            },
                                            border_color: Color::RED.into(),
                                            ..default()
                                        });
                                        count += 1;
                                    }
                                });
                        });
                });
        });
}

fn game_prepare_btn_action() {}
