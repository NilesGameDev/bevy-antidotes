use bevy::prelude::*;

use crate::{
    core::{despawn_entities, states::GameState},
    npc::{
        badcell, cell,
        goodcell::{self, GoodCell},
    },
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            (goodcell::spawn_good_cells, badcell::spawn_bad_cells),
        )
        .add_systems(
            Update,
            (cell::destroy_cell, badcell::move_attack, game_loop).run_if(in_state(GameState::Game)),
        )
        .add_systems(OnExit(GameState::Game), despawn_entities::<OnGameScreen>);
    }
}

#[derive(Component)]
pub struct OnGameScreen;

fn game_loop(query: Query<&GoodCell>, mut game_state: ResMut<NextState<GameState>>) {
    if query.is_empty() {
        game_state.set(GameState::GameOver);
    }
}
