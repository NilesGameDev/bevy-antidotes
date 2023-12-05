use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    Splash,
    #[default]
    Menu,
    Prepare,
    Game,
    GameOver,
    PostGame
}

