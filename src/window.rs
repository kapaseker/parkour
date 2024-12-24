use bevy::prelude::*;
use bevy::window::{WindowMode};
use bevy_rapier3d::prelude::*;

pub struct AppWindowPlugin;

impl Plugin for AppWindowPlugin {
    fn build(&self, app: &mut App) {
        let window = if cfg!(debug_assertions) {
            Window {
                resolution: (1200.0, 1200.0).into(),
                ..default()
            }
        } else {
            Window {
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                ..default()
            }
        };

        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        })).add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default());
    }
}