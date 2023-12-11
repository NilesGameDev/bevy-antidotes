use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::{collide_aabb::collide, MaterialMesh2dBundle},
};
use rand::Rng;

use crate::plugins::{game::OnGameScreen, playerresource::PlayerResource};

use super::{
    cell::{Cell, CellAttack, CellAttribute, Collider},
    goodcell::GoodCell,
};

// TODO: refactor constants to components or configs or resources instead?
const BAD_CELL_SPAWN_RADIUS: f32 = 400.0;
const BAD_CELL_SEARCH_RADIUS: f32 = 1000.0;
const BAD_CELL_ATTACK_RANGE: f32 = 10.0;
const BAD_CELL_BASE_COUNT: i32 = 60;
const BAD_CELL_BASE_STRENGTH_MULTIPLIER: f32 = 0.25;

#[derive(Component)]
pub struct BadCell;
#[derive(Component)]
pub struct SearchRange {
    pub range: f32,
}

pub fn spawn_bad_cells(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut animations: ResMut<Assets<AnimationClip>>,
    player_resource: ResMut<PlayerResource>
) {
    let current_wave = player_resource.wave_num;
    let mut cell_count = 0;
    let max_cell_count = BAD_CELL_BASE_COUNT + current_wave * 3;
    let bad_cell_strength_multiplier = if current_wave != 0 && current_wave % 3 == 0 {
        current_wave as f32 * BAD_CELL_BASE_STRENGTH_MULTIPLIER
    } else {
        1.0
    };

    while cell_count < max_cell_count {
        let mut animation = AnimationClip::default();
        let mut player = AnimationPlayer::default();
        let mut origin_point = Vec3::new(0., 0., 0.);
        origin_point.x =
            rand::thread_rng().gen_range(-BAD_CELL_SPAWN_RADIUS..=BAD_CELL_SPAWN_RADIUS);
        origin_point.y =
            rand::thread_rng().gen_range(-BAD_CELL_SPAWN_RADIUS..=BAD_CELL_SPAWN_RADIUS);

        let origin_point = origin_point.normalize() * rand::thread_rng().gen_range(300.0..=320.);

        // TODO: refactor the below code
        let anim_cell = Name::new(format!("anim_cell_{cell_count}"));
        let child_origin = Vec3::new(0., 0., 0.);

        let rand_keyframe_1 = Vec3::new(
            rand::thread_rng().gen_range(-5.0..=5.0),
            rand::thread_rng().gen_range(-5.0..=5.0),
            0.0,
        );
        let rand_keyframe_2 = Vec3::new(
            rand::thread_rng().gen_range(-5.0..=5.0),
            rand::thread_rng().gen_range(-5.0..=5.0),
            0.0,
        );
        let rand_keyframe_3 = Vec3::new(
            rand::thread_rng().gen_range(-5.0..=5.0),
            rand::thread_rng().gen_range(-5.0..=5.0),
            0.0,
        );

        animation.add_curve_to_path(
            EntityPath {
                parts: vec![anim_cell.clone()],
            },
            VariableCurve {
                keyframe_timestamps: vec![0.0, 1.0, 2.0, 3.0, 4.0],
                keyframes: Keyframes::Translation(vec![
                    child_origin,
                    child_origin + rand_keyframe_1,
                    child_origin + rand_keyframe_2,
                    child_origin + rand_keyframe_3,
                    // in case seamless looping is wanted, the last keyframe should
                    // be the same as the first one
                    child_origin,
                ]),
            },
        );

        player.play(animations.add(animation)).repeat();

        let modify_speed = 5.7 - bad_cell_strength_multiplier * 0.7;
        let modify_damage = 0.8 + bad_cell_strength_multiplier * 0.9;

        // TODO: clean up unused components!!!
        commands
            .spawn((
                SpatialBundle::from_transform(Transform::from_translation(origin_point)),
                Cell,
                BadCell,
                CellAttribute {
                    health: 10.0 * bad_cell_strength_multiplier,
                    immune: 100.0,
                    infection: 0.0,
                    cell_attack: CellAttack::new(modify_speed, modify_damage),
                },
                Collider,
                SearchRange {
                    range: BAD_CELL_SEARCH_RADIUS,
                },
                OnGameScreen, // TODO: find a better way to add this component to a cell
            ))
            .with_children(|child_builder| {
                child_builder.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::RED)),
                        transform: Transform::from_translation(child_origin),
                        ..default()
                    },
                    anim_cell,
                    player,
                ));
            });

        cell_count += 1;
    }
}

pub fn move_attack(
    time: ResMut<Time>,
    mut badcell_query: Query<
        (&mut Transform, &SearchRange, &mut CellAttribute),
        (With<BadCell>, With<OnGameScreen>),
    >,
    mut collision_query: Query<
        (&Transform, &GoodCell, &mut CellAttribute),
        (With<Collider>, Without<BadCell>),
    >,
) {
    let target_pos = Vec3::new(0., 0., 0.);
    for (mut bad_cell_trans, cell_search_range, mut badcell_attr) in badcell_query.iter_mut() {
        let mut direction = (target_pos - bad_cell_trans.translation).normalize();
        let rand_speed = rand::thread_rng().gen_range(5.0..=80.0);
        let mut closest_distance_to_good_cell: f32 = f32::MAX;

        for (good_cell_trans, good_cell, mut goodcell_attr) in collision_query.iter_mut() {
            if Vec3::distance(good_cell_trans.translation, bad_cell_trans.translation)
                <= BAD_CELL_ATTACK_RANGE
            {
                let attack_rate = badcell_attr.cell_attack.attack_rate;
                badcell_attr
                    .cell_attack
                    .timer
                    .tick(Duration::from_secs_f32(attack_rate));
                if badcell_attr.cell_attack.timer.finished() {
                    let damage = badcell_attr.cell_attack.damage;
                    goodcell_attr.inflict_dmg(damage);

                    // TODO: refactor this
                    let infect_proc_chance = rand::thread_rng().gen_range(1..=100);
                    if infect_proc_chance <= 5 {
                        goodcell_attr.infect(2.0);
                    }
                }
            } else {
                let maybe_collide = collide(
                    bad_cell_trans.translation,
                    Vec2::new(cell_search_range.range, cell_search_range.range),
                    good_cell_trans.translation,
                    Vec2::new(good_cell.cell_size, good_cell.cell_size),
                );

                let distance =
                    Vec3::distance(good_cell_trans.translation, bad_cell_trans.translation);

                if maybe_collide.is_some() && distance <= closest_distance_to_good_cell {
                    direction =
                        (good_cell_trans.translation - bad_cell_trans.translation).normalize();
                    closest_distance_to_good_cell = distance;
                }
            }
        }

        bad_cell_trans.translation += direction * rand_speed * time.delta_seconds();
    }
}
