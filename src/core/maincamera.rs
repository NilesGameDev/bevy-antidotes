use bevy::{ecs::system::Commands, core_pipeline::core_2d::Camera2dBundle};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}