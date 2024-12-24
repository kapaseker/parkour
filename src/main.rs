mod light;
mod board;
mod window;
mod player;
mod constant;

use crate::board::AppBoardPlugin;
use crate::light::AppLightPlugin;
use crate::player::PlayerPlugin;
use crate::window::AppWindowPlugin;
use bevy::prelude::*;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins((AppWindowPlugin, PlayerPlugin, AppLightPlugin, AppBoardPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup() {
    info!("Start up game")
}
