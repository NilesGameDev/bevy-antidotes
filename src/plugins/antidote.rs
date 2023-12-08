use bevy::prelude::*;

pub struct AntidotePlugin;

impl Plugin for AntidotePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_substances);
    }
}

#[derive(Default, Clone)]
pub enum SubstanceType {
    #[default]
    Sweet,
    Bitter,
    Balanced,
}

#[derive(Default, Clone)]
pub enum TargetAttribute {
    Attack,
    Speed,
    #[default]
    Immune,
    Health
}

#[derive(Component, Default, Clone)] // TODO: or may be a resource?
pub struct Substance {
    pub id: i32,
    pub name: String,
    pub target_attribute: TargetAttribute,
    pub value: f32,
    pub substance_type: SubstanceType,
}

#[derive(Resource)]
pub struct SubstanceIdGen(pub i32);

#[derive(Resource)]
pub struct SubstanceResource(pub Vec<Substance>);

fn setup_substances(mut commands: Commands) {
    let substance_resources = vec![
        Substance {
            name: "Atagen".to_string(),
            target_attribute: TargetAttribute::Attack,
            value: 0.0,
            ..default()
        },
        Substance {
            name: "Speegen".to_string(),
            target_attribute: TargetAttribute::Speed,
            value: 0.0,
            ..default()
        },
        Substance {
            name: "Immugen".to_string(),
            target_attribute: TargetAttribute::Immune,
            value: 0.0,
            ..default()
        },
        Substance {
            name: "Helagen".to_string(),
            target_attribute: TargetAttribute::Health,
            value: 0.0,
            ..default()
        },
    ];
    commands.insert_resource(SubstanceResource(substance_resources));
    commands.insert_resource(SubstanceIdGen(0));
}
