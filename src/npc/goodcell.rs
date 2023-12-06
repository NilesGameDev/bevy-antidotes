use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::plugins::{game::OnGameScreen, playerresource::PlayerResource};

use super::cell::{Cell, CellAttribute, Collider, CellAttack};

const GOOD_CELL_SPAWN_RADIUS: f32 = 200.0;
const GOOD_CELL_SIZE: f32 = 15.0;

#[derive(Component)]
pub struct GoodCell {
    pub cell_size: f32, // TODO: move to the Cell struct instead?
}

pub fn spawn_good_cells(
    mut commands: Commands,
    _: Res<PlayerResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    let mut cell_count = 0;

    while cell_count < 20 {
        let mut animation = AnimationClip::default();
        let mut player = AnimationPlayer::default();
        let mut origin_point = Vec3::new(0., 0., 0.);
        origin_point.x =
            rand::thread_rng().gen_range(-GOOD_CELL_SPAWN_RADIUS..=GOOD_CELL_SPAWN_RADIUS);
        origin_point.y =
            rand::thread_rng().gen_range(-GOOD_CELL_SPAWN_RADIUS..=GOOD_CELL_SPAWN_RADIUS);

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
                cell_size: GOOD_CELL_SIZE,
            },
            CellAttribute {
                health: 100.0,
                immune: 100.0,
                infection: 0.0,
                cell_attack: CellAttack::new(1.0, 50.0)
            },
            Collider,
            OnGameScreen // TODO: find a better way to add this component to a cell
        ));

        cell_count += 1;
    }
}
