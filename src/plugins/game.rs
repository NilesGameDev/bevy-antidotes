use bevy::prelude::*;

use crate::{
    core::{despawn_entities, states::GameState},
    npc::{
        badcell::{self, BadCell},
        cell,
        goodcell::{self, GoodCell},
    },
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            (
                setup_ingame_resources,
                goodcell::spawn_good_cells,
                badcell::spawn_bad_cells,
            ),
        )
        .add_systems(
            Update,
            (
                cell::destroy_cell,
                cell::track_cell_infection,
                goodcell::attack,
                badcell::move_attack,
                game_loop,
            )
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(OnExit(GameState::GameFinish), despawn_entities::<OnGameScreen>);
    }
}

#[derive(Component)]
pub struct OnGameScreen;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn setup_ingame_resources(mut commands: Commands) {
    commands.insert_resource(GameTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

fn game_loop(
    time: Res<Time>,
    goodcell_query: Query<&GoodCell>,
    badcell_query: Query<&BadCell>,
    mut game_state: ResMut<NextState<GameState>>,
    mut timer: ResMut<GameTimer>
) {
    if goodcell_query.is_empty() {
        game_state.set(GameState::GameOver);
    }

    if badcell_query.is_empty() {
        if timer.tick(time.delta()).finished() {
            game_state.set(GameState::GameFinish);
        }
    }
}
