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

#[derive(Component, Default, Clone)] // TODO: or may be a resource?
pub struct Substance {
    pub name: String,
    pub target_attribute: String,
    pub value: f32,
    pub substance_type: SubstanceType,
}

#[derive(Resource)]
pub struct SubstanceResource(pub Vec<Substance>);

fn setup_substances(mut commands: Commands) {
    let substance_resources = vec![
        Substance {
            name: "Atagen".to_string(),
            target_attribute: "attack".to_string(),
            value: 0.0,
            ..default()
        },
        Substance {
            name: "Speegen".to_string(),
            target_attribute: "attack_rate".to_string(),
            value: 0.0,
            ..default()
        },
        Substance {
            name: "Immugen".to_string(),
            target_attribute: "immune".to_string(),
            value: 0.0,
            ..default()
        },
        Substance {
            name: "Helagen".to_string(),
            target_attribute: "health".to_string(),
            value: 0.0,
            ..default()
        },
    ];
    commands.insert_resource(SubstanceResource(substance_resources));
}
