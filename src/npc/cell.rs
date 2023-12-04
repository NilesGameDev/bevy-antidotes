use bevy::prelude::*;

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct CellAttribute {
    pub health: f32,
    pub immune_rate: i32,
}

impl CellAttribute {
    pub fn inflict_dmg(&mut self, damage: &f32) {
        self.health -= damage;
    }
}

#[derive(Component)]
pub struct Cell;

#[derive(Component)]
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
    mut query: Query<(Entity, &CellAttribute), With<Cell>>,
) {
    for (ent, cell_attr) in query.iter_mut() {
        if cell_attr.health <= 0.0 {
            commands.entity(ent).despawn();
        }
    }
}
