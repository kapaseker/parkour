use bevy::prelude::*;
use parkour::board::AppBoardPlugin;
use parkour::ecs_close::CloseOnEscPlugin;
use parkour::light::AppLightPlugin;
use parkour::music::BackgroundMusicPlugin;
use parkour::player::KnightPlugin;
use parkour::window::AppWindowPlugin;

fn main() {
    #[cfg(target_os = "windows")]
    {
        std::env::set_var("WGPU_BACKEND", "dx12");
    }
    App::new()
        .add_plugins((AppWindowPlugin, KnightPlugin, AppLightPlugin, AppBoardPlugin, CloseOnEscPlugin, BackgroundMusicPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup() {
    info!("Start up game")
}
