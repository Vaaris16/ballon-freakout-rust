use bevy::{prelude::*, window::WindowResolution};

use crate::{
    background::background_plugin::BackgroundPlugin, score::score_plugin::ScorePlugin,
    spawn_player::spawn_player_plugin::SpawnPlayerPlugin,
};

mod background;
mod score;
mod spawn_player;

const SCREEN_WIDTH: u32 = 700;
const SCREEN_HEIGHT: u32 = 800;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((BackgroundPlugin, SpawnPlayerPlugin, ScorePlugin))
        .run();
}
