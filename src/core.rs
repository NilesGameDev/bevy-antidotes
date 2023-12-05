use bevy::prelude::*;

pub mod states;
pub mod maincamera;
pub mod physics;
pub mod userinterface;

pub fn despawn_entities<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}