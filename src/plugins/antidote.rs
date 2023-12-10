use bevy::prelude::*;
use std::fmt;

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

impl fmt::Display for SubstanceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubstanceType::Balanced => write!(f, "Balanced"),
            SubstanceType::Sweet => write!(f, "Sweet"),
            SubstanceType::Bitter => write!(f, "Bitter"),
        }
    }
}

#[derive(Default, Clone)]
pub enum TargetAttribute {
    Attack,
    Speed,
    #[default]
    Immune,
    Health,
}

impl fmt::Display for TargetAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetAttribute::Attack => write!(f, "Attack"),
            TargetAttribute::Speed => write!(f, "Attack per sec"),
            TargetAttribute::Immune => write!(f, "Immune"),
            TargetAttribute::Health => write!(f, "Health"),
        }
    }
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
}
