use std::collections::HashMap;

use bevy::prelude::*;

use crate::npc::{goodcell::GoodCellId, cell::CellBundle};

use super::antidote::Substance;

pub struct PlayerResourcePlugin;

impl Plugin for PlayerResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player_resources);
    }
}

#[derive(Resource)]
pub struct PlayerResource {
    pub cell_army: HashMap<i32, CellBundle>,
    pub substance_collection: HashMap<i32, Substance>,
    pub loaded_substances: HashMap<i32, Substance>,
    pub good_cell_id: GoodCellId,
    pub wave_num: i32,
    pub substance_id_gen: SubstanceIdGen
}

#[derive(Resource)]
pub struct SubstanceIdGen(pub i32);

fn setup_player_resources(mut commands: Commands) {
    let cell_army: HashMap<i32, CellBundle> = HashMap::new();
    let substance_collection: HashMap<i32, Substance> = HashMap::new();
    let loaded_substances: HashMap<i32, Substance> = HashMap::new();
    let good_cell_id = GoodCellId(0);
    let wave_num = 0;
    let substance_id_gen = SubstanceIdGen(0);

    commands.insert_resource(PlayerResource {
        cell_army,
        substance_collection,
        loaded_substances,
        good_cell_id,
        wave_num,
        substance_id_gen
    });
}
