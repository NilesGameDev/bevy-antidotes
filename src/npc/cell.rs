use bevy::ecs::component::Component;

#[derive(Component)]
pub struct CellAttribute {
    pub health: f32,
    pub immune_rate: i32
}

#[derive(Component)]
pub struct Cell;
