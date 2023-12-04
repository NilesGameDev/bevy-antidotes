use antidotes::{
    core::maincamera,
    npc::{badcell, cell, goodcell},
};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                maincamera::setup_camera,
                goodcell::spawn_good_cells,
                badcell::spawn_bad_cells,
            ),
        )
        .add_systems(Update, (cell::destroy_cell, badcell::move_attack))
        .run();
}
