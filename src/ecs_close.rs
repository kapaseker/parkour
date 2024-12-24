use bevy::app::{App, Plugin, Update};
use bevy::input::ButtonInput;
use bevy::prelude::{Commands, Entity, KeyCode, Query, Res};
use bevy::window::Window;

#[derive(Debug, Default)]
pub struct CloseOnEscPlugin;

impl Plugin for CloseOnEscPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, close_on_esc);
    }
}

fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

