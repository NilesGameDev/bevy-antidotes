use bevy::prelude::*;

use crate::core;
use crate::core::{
    states::GameState,
    userinterface::{GAME_THEME_COLOR, NORMAL_BUTTON},
};
use crate::npc::cell::CellAttribute;
use crate::npc::goodcell::GoodCell;

use super::playerresource::PlayerResource;

pub struct GameFinishPlugin;

impl Plugin for GameFinishPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameFinish),
            (setup_game_finish_screen, update_wave_clear),
        )
        .add_systems(
            Update,
            game_finish_action.run_if(in_state(GameState::GameFinish)),
        )
        .add_systems(
            OnExit(GameState::GameFinish),
            core::despawn_entities::<OnGameFinishScreen>,
        );
    }
}

#[derive(Component)]
struct OnGameFinishScreen;

#[derive(Component)]
enum GameFinishButtonAction {
    ReturnToMainMenu,
    NextRound,
}

fn setup_game_finish_screen(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(180.0),
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
                    flex_direction: FlexDirection::Column,
                    align_content: AlignContent::Center,
                    ..default()
                },
                ..default()
            },
            OnGameFinishScreen,
        ))
        .with_children(|parent| {
            let bg_color = GAME_THEME_COLOR.clone().set_a(0.6).as_rgba();
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(85.0),
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: bg_color.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Wave Cleared!",
                        TextStyle {
                            font_size: 80.0,
                            ..default()
                        },
                    ));
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(15.0),
                        justify_content: JustifyContent::FlexEnd,
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
                            GameFinishButtonAction::ReturnToMainMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Main Menu",
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
                            GameFinishButtonAction::NextRound,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Next Round",
                                button_txt_style.clone(),
                            ));
                        });
                });
        });
}

fn update_wave_clear(mut player_resources: ResMut<PlayerResource>, good_cell_query: Query<(&CellAttribute, &GoodCell)>) {
    player_resources.wave_num += 1;

    // Update the fight statistic to player resources
    for (good_cell_attr, good_cell) in good_cell_query.iter() {
        if let Some(old_cell_bundle) = player_resources.cell_army.get_mut(&good_cell.cell_id) {
            old_cell_bundle.cell_attribute = good_cell_attr.clone();
        }
    }
}

fn game_finish_action(
    interaction_query: Query<
        (&Interaction, &GameFinishButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, game_over_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match game_over_button_action {
                GameFinishButtonAction::ReturnToMainMenu => {
                    game_state.set(GameState::Menu);
                }
                GameFinishButtonAction::NextRound => {
                    game_state.set(GameState::Prepare);
                }
            }
        }
    }
}
