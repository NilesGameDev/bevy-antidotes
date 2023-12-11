use antidotes::{
    core::{maincamera, states, userinterface},
    plugins::{antidote, game, gameover, menu, playerresource, gameprepare, gamefinish},
};
use bevy::{prelude::*, asset::AssetMetaCheck};

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
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
