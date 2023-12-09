use bevy::prelude::*;

pub const GAME_THEME_COLOR: Color = Color::hsl(160.0, 0.26, 0.54);
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Resource)]
pub struct UiResource(Handle<AudioSource>);

#[derive(Event, Default)]
pub struct ButtonClickEvent;

pub fn setup_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_click_sound = asset_server.load("audio/game-ui/buttons/bubbles-single2.wav");
    commands.insert_resource(UiResource(button_click_sound));
}

pub fn button_systems(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut btn_click_events: EventWriter<ButtonClickEvent>,
) {
    for (interaction, mut color) in &mut interaction_query {
        *color = match *interaction {
            Interaction::Pressed => {
                btn_click_events.send_default();
                PRESSED_BUTTON.into()
            },
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        }
    }
}

pub fn play_button_click_sound(
    mut commands: Commands,
    mut btn_click_events: EventReader<ButtonClickEvent>,
    sound: Res<UiResource>,
) {
    if !btn_click_events.is_empty() {
        btn_click_events.clear();
        commands.spawn(AudioBundle {
            source: sound.0.clone(),
            // auto-despawn the entity when playback finishes
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
