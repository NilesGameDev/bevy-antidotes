use bevy::prelude::*;

use crate::core;
use crate::core::states::GameState;
use crate::core::userinterface::{GAME_THEME_COLOR, NORMAL_BUTTON};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup_game_over_screen)
            .add_systems(
                Update,
                game_over_action.run_if(in_state(GameState::GameOver)),
            )
            .add_systems(
                OnExit(GameState::GameOver),
                core::despawn_entities::<OnGameOverScreen>,
            );
    }
}

#[derive(Component)]
struct OnGameOverScreen;

#[derive(Component)]
enum GameOverButtonAction {
    ReturnToMainMenu, //TODO: more action coming soon!
}

fn setup_game_over_screen(mut commands: Commands) {
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
                ..default()
            },
            OnGameOverScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::MIDNIGHT_BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "You are INFECTED!",
                            TextStyle {
                                font_size: 70.0,
                                color: GAME_THEME_COLOR.into(),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(8.0)),
                            ..default()
                        }),
                    );
                    parent.spawn(
                        TextBundle::from_section(
                            "Game Over",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(5.0)),
                            ..default()
                        }),
                    );
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            GameOverButtonAction::ReturnToMainMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Return To Main Menu",
                                button_txt_style.clone(),
                            ));
                        });
                });
        });
}

fn game_over_action(
    interaction_query: Query<
        (&Interaction, &GameOverButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, game_over_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match game_over_button_action {
                GameOverButtonAction::ReturnToMainMenu => {
                    game_state.set(GameState::Menu);
                }
            }
        }
    }
}
