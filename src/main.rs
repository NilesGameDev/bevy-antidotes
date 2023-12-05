use antidotes::{
    core::{maincamera, states, userinterface},
    plugins::{game, gameover, menu},
};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<userinterface::ButtonClickEvent>()
        .add_state::<states::GameState>()
        .add_systems(
            Startup,
            (maincamera::setup_camera, userinterface::setup_resources),
        )
        .add_systems(
            Update,
            (
                userinterface::button_systems,
                userinterface::play_button_click_sound,
            ),
        )
        .add_plugins((menu::MenuPlugin, game::GamePlugin, gameover::GameOverPlugin))
        .run();
}
