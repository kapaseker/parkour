use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct BackgroundMusicPlugin;

impl Plugin for BackgroundMusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin).add_systems(Startup, setup_background_music);
    }
}

fn setup_background_music(
    audio: Res<Audio>,
    asset_server: Res<AssetServer>
) {
    audio.play(asset_server.load("sound/background.mp3")).looped();
}
