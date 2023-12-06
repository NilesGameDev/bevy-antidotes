use bevy::{app::AppExit, prelude::*};

use crate::core;
use crate::core::states::GameState;
use crate::core::userinterface::{GAME_THEME_COLOR, NORMAL_BUTTON};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(OnEnter(MenuState::Main), setup_main_menu)
            .add_systems(
                OnExit(MenuState::Main),
                core::despawn_entities::<OnMainMenuScreen>,
            )
            .add_systems(Update, menu_action.run_if(in_state(GameState::Menu)));
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, Hash, States)]
enum MenuState {
    Main,
    #[default]
    Disabled,
}

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

#[derive(Component)]
struct OnMainMenuScreen;

fn setup_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn setup_main_menu(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_txt_style = TextStyle {
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
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
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::AQUAMARINE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Antidotes",
                            TextStyle {
                                font_size: 80.0,
                                color: GAME_THEME_COLOR.into(),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
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
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "New Game",
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
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Quit", button_txt_style));
                        });
                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => {
                    game_state.set(GameState::Prepare);
                    menu_state.set(MenuState::Disabled);
                }
            }
        }
    }
}
