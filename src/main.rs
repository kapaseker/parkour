use bevy::prelude::*;
use parkour::board::AppBoardPlugin;
use parkour::ecs_close::CloseOnEscPlugin;
use parkour::light::AppLightPlugin;
use parkour::player::PlayerPlugin;
use parkour::window::AppWindowPlugin;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins((AppWindowPlugin, PlayerPlugin, AppLightPlugin, AppBoardPlugin, CloseOnEscPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup() {
    info!("Start up game")
}
