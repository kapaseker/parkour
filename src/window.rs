use bevy::prelude::*;
use bevy::window::{WindowMode};
use bevy_rapier3d::prelude::*;

pub struct AppWindowPlugin;

impl Plugin for AppWindowPlugin {
    fn build(&self, app: &mut App) {

        let window = Window {
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            ..default()
        };

        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        })).add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default());
    }
}