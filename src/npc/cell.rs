use bevy::prelude::*;
use bevy_hanabi::{ParticleEffect, ParticleEffectBundle};
use rand::Rng;

use crate::{
    core::particlesystem::CellExplodeEffectRes,
    plugins::{
        antidote::{SubstanceResource, SubstanceType, TargetAttribute},
        game::{CollectedSubstanceDisplay, OnGameScreen},
        playerresource::PlayerResource,
    },
};

use super::{badcell::BadCell, goodcell::GoodCell};

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
    explode_vfx_res: Res<CellExplodeEffectRes>,
    substance_resources: Res<SubstanceResource>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut player_resources: ResMut<PlayerResource>,
    mut query: Query<
        (
            Entity,
            &Transform,
            &CellAttribute,
            Option<&BadCell>,
            Option<&GoodCell>,
        ),
        With<Cell>,
    >,
    collected_sub_display_query: Query<(&Transform, With<CollectedSubstanceDisplay>)>,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    let (collected_sub_display_trans, _) = collected_sub_display_query.single();
    let sub_attack_img: Handle<Image> = asset_server.load("sprites/sub_attack.png");
    let sub_speed_img: Handle<Image> = asset_server.load("sprites/sub_speed.png");
    let sub_immune_img: Handle<Image> = asset_server.load("sprites/sub_immune.png");
    let sub_health_img: Handle<Image> = asset_server.load("sprites/sub_health.png");

    let mut temp = 0;
    for (ent, cell_trans, cell_attr, maybe_badcell, maybe_goodcell) in query.iter_mut() {
        if cell_attr.health <= 0.0 {
            if maybe_badcell.is_some() {
                let drop_chance = rand::thread_rng().gen_range(1..=100);
                if drop_chance <= 10 {
                    let resource_len = substance_resources.0.len();
                    let random_substance_idx = rand::thread_rng().gen_range(0..resource_len);

                    let mut random_substance = substance_resources.0[random_substance_idx].clone();
                    random_substance.id = player_resources.substance_id_gen.0;
                    random_substance.value = match random_substance.target_attribute {
                        TargetAttribute::Speed => rand::thread_rng().gen_range(-0.4..=0.2),
                        _ => rand::thread_rng().gen_range(-5.0..=8.0),
                    };
                    random_substance.substance_type = if random_substance.value < 0.0 {
                        SubstanceType::Bitter
                    } else if random_substance.value == 0.0 {
                        SubstanceType::Balanced
                    } else {
                        SubstanceType::Sweet
                    };
                    player_resources
                        .substance_collection
                        .insert(random_substance.id, random_substance.clone());
                    player_resources.substance_id_gen.0 += 1;

                    //TODO: refactor this
                    let substance_sprite = match random_substance.target_attribute {
                        TargetAttribute::Attack => sub_attack_img.clone(),
                        TargetAttribute::Health => sub_health_img.clone(),
                        TargetAttribute::Immune => sub_immune_img.clone(),
                        TargetAttribute::Speed => sub_speed_img.clone(),
                    };
                    let mut animation = AnimationClip::default();
                    let mut player = AnimationPlayer::default();
                    let anim_sub = Name::new(format!("anim_sub_{temp}"));
                    temp += 1;
                    println!("{}", cell_trans.translation);
                    animation.add_curve_to_path(
                        EntityPath {
                            parts: vec![anim_sub.clone()],
                        },
                        VariableCurve {
                            keyframe_timestamps: vec![0.0, 5.0, 10.0],
                            keyframes: Keyframes::Translation(vec![
                                cell_trans.translation,
                                cell_trans.translation
                                    + 100.0 * Vec3::new(0.0, 1.0, 0.0),
                                collected_sub_display_trans.translation,
                            ]),
                        },
                    );

                    player.play(animations.add(animation)).repeat();
                    commands
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Auto,
                                    height: Val::Auto,
                                    ..default()
                                },
                                transform: cell_trans.clone(),
                                ..default()
                            },
                            OnGameScreen,
                            player,
                            anim_sub,
                        ))
                        .with_children(|parent| {
                            parent.spawn((ImageBundle {
                                style: Style {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    ..default()
                                },
                                image: UiImage::new(substance_sprite),
                                ..default()
                            },));
                        });
                }

                commands.spawn((
                    ParticleEffectBundle {
                        effect: ParticleEffect::new(explode_vfx_res.0.clone()),
                        transform: *cell_trans,
                        ..Default::default()
                    },
                    OnGameScreen,
                ));
            } else if let Some(maybe_goodcell) = maybe_goodcell {
                player_resources.cell_army.remove(&maybe_goodcell.cell_id);
            }

            commands.entity(ent).despawn_recursive();
        }
    }
}

pub fn track_cell_infection(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(
        Entity,
        &GoodCell,
        &mut CellAttribute,
        &Handle<ColorMaterial>,
    )>,
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
