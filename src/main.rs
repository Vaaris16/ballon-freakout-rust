use bevy::{prelude::*, window::WindowResolution};

use crate::{background::background_plugin::BackgroundPlugin, game::game::GamePlugin};

mod background;
mod game;

const SCREEN_WIDTH: u32 = 700;
const SCREEN_HEIGHT: u32 = 800;

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    #[default]
    Playing,
    Restart,
}

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
        .init_state::<GameState>()
        .add_plugins((BackgroundPlugin, GamePlugin))
        .run();
}
