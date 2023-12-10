use antidotes::{
    core::{maincamera, states, userinterface, particlesystem},
    plugins::{antidote, game, gameover, menu, playerresource, gameprepare, gamefinish},
};
use bevy::prelude::*;
use bevy_hanabi::HanabiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HanabiPlugin)
        .add_event::<userinterface::ButtonClickEvent>()
        .add_state::<states::GameState>()
        .add_systems(
            Startup,
            (maincamera::setup_camera, userinterface::setup_resources, particlesystem::setup),
        )
        .add_systems(
            Update,
            (
                userinterface::button_systems,
                userinterface::play_button_click_sound,
            ),
        )
        .add_plugins((
            antidote::AntidotePlugin,
            playerresource::PlayerResourcePlugin,
            menu::MenuPlugin,
            gameprepare::GamePreparePlugin,
            game::GamePlugin,
            gamefinish::GameFinishPlugin,
            gameover::GameOverPlugin,
        ))
        .run();
}
