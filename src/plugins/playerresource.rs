use bevy::prelude::*;

use crate::npc::goodcell::GoodCell;

use super::antidote::Substance;

pub struct PlayerResourcePlugin;

impl Plugin for PlayerResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player_resources);
    }
}

#[derive(Resource)]
pub struct PlayerResource {
    pub cell_army: Vec<GoodCell>,
    pub substance_collection: Vec<Substance>,
}

fn setup_player_resources(mut commands: Commands) {
    let cell_army: Vec<GoodCell> = vec![];
    let substance_collection: Vec<Substance> = vec![];

    commands.insert_resource(PlayerResource {
        cell_army,
        substance_collection,
    });
}
