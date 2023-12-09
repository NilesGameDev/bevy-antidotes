use bevy::prelude::*;
use rand::Rng;

use crate::plugins::{playerresource::PlayerResource, antidote::{SubstanceResource, SubstanceType, SubstanceIdGen}};

use super::{goodcell::GoodCell, badcell::BadCell};

#[derive(Component)]
pub struct Collider;

#[derive(Component, Clone)]
pub struct CellAttribute {
    pub health: f32,
    pub immune: f32,
    pub cell_attack: CellAttack,
    pub infection: f32,
}

impl CellAttribute {
    pub fn inflict_dmg(&mut self, damage: f32) {
        self.health -= damage;
        self.health = f32::max(0.0, self.health);
    }

    pub fn infect(&mut self, rate: f32) {
        self.infection += rate;
    }
}

#[derive(Component)]
pub struct Cell;

#[derive(Component, Clone)]
pub struct CellAttack {
    pub attack_rate: f32, // as seconds
    pub damage: f32,
    pub timer: Timer,
}

impl CellAttack {
    pub fn new(attack_rate: f32, damage: f32) -> Self {
        Self {
            attack_rate,
            damage,
            timer: Timer::from_seconds(attack_rate, TimerMode::Repeating),
        }
    }
}

//TODO: find a better way to handle a cell being destroyed as we can mutate the cell instead
// for example: making bad cell turn "good" and vice versa
pub fn destroy_cell(
    mut commands: Commands,
    mut playerresource: ResMut<PlayerResource>,
    substance_resources: Res<SubstanceResource>,
    mut substance_id_gen_res: ResMut<SubstanceIdGen>,
    mut query: Query<(Entity, &CellAttribute, Option<&BadCell>), With<Cell>>,
) {
    for (ent, cell_attr, maybe_badcell) in query.iter_mut() {
        if cell_attr.health <= 0.0 {
            if maybe_badcell.is_some() {
                let drop_chance = rand::thread_rng().gen_range(1..=100);
                if drop_chance <= 10 {
                    let resource_len = substance_resources.0.len();
                    let random_substance_idx = rand::thread_rng().gen_range(0..resource_len);
                    
                    let mut random_substance = substance_resources.0[random_substance_idx].clone();
                    random_substance.id = substance_id_gen_res.0;
                    random_substance.value = rand::thread_rng().gen_range(-8..=15) as f32;
                    random_substance.substance_type = if random_substance.value < 0.0 {
                        SubstanceType::Bitter
                    } else if random_substance.value == 0.0 {
                        SubstanceType::Balanced
                    } else {
                        SubstanceType::Sweet
                    };
                    playerresource.substance_collection.insert(substance_id_gen_res.0, random_substance);
                    substance_id_gen_res.0 += 1;
                }
            }
            commands.entity(ent).despawn_recursive();
        }
    }
}

pub fn track_cell_infection(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &GoodCell, &mut CellAttribute, &Handle<ColorMaterial>)>,
) {
    for (ent, _, mut cell_attr, color_mat_handle) in query.iter_mut() {
        if cell_attr.infection <= cell_attr.immune {
            continue;
        }

        let golden_chance = rand::thread_rng().gen_range(1..=100);

        // there is small chance the cell will get stronger after infection
        // ref from Darkest Dungeon stress system!
        if golden_chance <= 30 {
            cell_attr.health += 100.0;
            cell_attr.infection = 0.0;
            cell_attr.immune = 90.0;
        } else {
            let color_mat = materials.get_mut(color_mat_handle).unwrap();
            color_mat.color = Color::RED;
            cell_attr.health = f32::max(25.0, cell_attr.health);
            cell_attr.cell_attack.damage = f32::min(5.0, cell_attr.cell_attack.damage);
            cell_attr.cell_attack.attack_rate = f32::max(8.0, cell_attr.cell_attack.attack_rate);
            commands.entity(ent).remove::<GoodCell>().insert(BadCell);
        }
    }
}
