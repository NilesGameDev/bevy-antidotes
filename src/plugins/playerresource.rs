use std::collections::HashMap;

use bevy::prelude::*;

use crate::npc::{goodcell::GoodCellId, cell::CellAttribute};

use super::antidote::{Substance, SubstanceIdGen, TargetAttribute, SubstanceType};

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
    let mut substance_collection: HashMap<i32, Substance> = HashMap::new();
    let loaded_substances: HashMap<i32, Substance> = HashMap::new();
    let good_cell_id = GoodCellId(0);
    let mut substance_id_gen_res = SubstanceIdGen(0);

    substance_collection.insert(substance_id_gen_res.0, Substance { 
        id: substance_id_gen_res.0, 
        name: "Initgen".to_string(), 
        target_attribute: TargetAttribute::Immune, 
        value: 50.0, 
        substance_type: SubstanceType::Balanced
    });
    substance_id_gen_res.0 += 1;

    substance_collection.insert(substance_id_gen_res.0, Substance { 
        id: substance_id_gen_res.0, 
        name: "Initgen".to_string(), 
        target_attribute: TargetAttribute::Immune, 
        value: 50.0, 
        substance_type: SubstanceType::Balanced
    });
    substance_id_gen_res.0 += 1;

    commands.insert_resource(PlayerResource {
        cell_army,
        substance_collection,
        loaded_substances,
        good_cell_id
    });
    commands.insert_resource(substance_id_gen_res);
}
