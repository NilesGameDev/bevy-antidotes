use std::collections::HashMap;

use bevy::prelude::*;

use crate::npc::{goodcell::GoodCellId, cell::CellAttribute};

use super::antidote::Substance;

pub struct PlayerResourcePlugin;

impl Plugin for PlayerResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player_resources);
    }
}

#[derive(Resource)]
pub struct PlayerResource {
    pub cell_army: HashMap<i32, CellAttribute>,
    pub substance_collection: HashMap<i32, Substance>,
    pub loaded_substances: HashMap<i32, Substance>,
    pub good_cell_id: GoodCellId
}

fn setup_player_resources(mut commands: Commands) {
    let cell_army: HashMap<i32, CellAttribute> = HashMap::new();
    let substance_collection: HashMap<i32, Substance> = HashMap::new();
    let loaded_substances: HashMap<i32, Substance> = HashMap::new();
    let good_cell_id = GoodCellId(0);

    commands.insert_resource(PlayerResource {
        cell_army,
        substance_collection,
        loaded_substances,
        good_cell_id
    });
}
