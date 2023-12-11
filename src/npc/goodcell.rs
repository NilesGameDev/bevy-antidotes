use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::plugins::{game::OnGameScreen, playerresource::PlayerResource};

use super::{
    badcell::BadCell,
    cell::{Cell, CellAttribute, Collider},
};

pub const GOOD_CELL_ATTACK_RANGE: f32 = 10.0;
pub const GOOD_CELL_SPAWN_RADIUS: f32 = 200.0;
pub const GOOD_CELL_SIZE: f32 = 15.0;

#[derive(Component)]
pub struct GoodCell {
    pub cell_id: i32, // TODO: move to the Cell struct instead?
    pub cell_size: f32,
}

#[derive(Resource)]
pub struct GoodCellId(pub i32);

pub fn spawn_good_cells(
    mut commands: Commands,
    mut player_resources: ResMut<PlayerResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    let mut cell_count = 0;

    for (id, good_cell_bundle) in player_resources.cell_army.iter_mut() {
        let good_cell_attr = &good_cell_bundle.cell_attribute;
        let mut animation = AnimationClip::default();
        let mut player = AnimationPlayer::default();
        let mut origin_point = good_cell_bundle.cell_trans;

        if origin_point == Vec3::ZERO {
            origin_point.x =
                rand::thread_rng().gen_range(-GOOD_CELL_SPAWN_RADIUS..=GOOD_CELL_SPAWN_RADIUS);
            origin_point.y =
                rand::thread_rng().gen_range(-GOOD_CELL_SPAWN_RADIUS..=GOOD_CELL_SPAWN_RADIUS);
        }

        // TODO: refactor the below code
        let anim_cell = Name::new(format!("anim_cell_{cell_count}"));

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
                    origin_point,
                    origin_point + rand_keyframe_1,
                    origin_point + rand_keyframe_2,
                    origin_point + rand_keyframe_3,
                    // in case seamless looping is wanted, the last keyframe should
                    // be the same as the first one
                    origin_point,
                ]),
            },
        );

        player.play(animations.add(animation)).repeat();

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(GOOD_CELL_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_translation(origin_point),
                ..default()
            },
            anim_cell,
            player,
            Cell,
            GoodCell {
                cell_id: id.clone(),
                cell_size: 15.0,
            },
            good_cell_attr.clone(),
            Collider,
            OnGameScreen, // TODO: find a better way to add this component to a cell
        ));

        cell_count += 1;
    }
}

pub fn attack(
    mut goodcell_query: Query<(&mut Transform, &mut CellAttribute), With<GoodCell>>,
    mut collision_query: Query<
        (&Transform, &mut CellAttribute),
        (With<Collider>, With<BadCell>, Without<GoodCell>),
    >,
) {
    for (good_cell_trans, mut goodcell_attr) in goodcell_query.iter_mut() {
        for (bad_cell_trans, mut badcell_attr) in collision_query.iter_mut() {
            if Vec3::distance(good_cell_trans.translation, bad_cell_trans.translation)
                <= GOOD_CELL_ATTACK_RANGE
            {
                let attack_rate = goodcell_attr.cell_attack.attack_rate;
                goodcell_attr
                    .cell_attack
                    .timer
                    .tick(Duration::from_secs_f32(attack_rate));
                if goodcell_attr.cell_attack.timer.finished() {
                    let damage = goodcell_attr.cell_attack.damage;
                    badcell_attr.inflict_dmg(damage);
                }
            }
        }
    }
}
