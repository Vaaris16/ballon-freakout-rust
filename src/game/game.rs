use bevy::prelude::*;

use crate::game::{
    score::score_plugin::ScorePlugin, spawn_main_sprite::spawn::SpawnMainSpritePlugin,
    spawn_player::spawn_player_plugin::SpawnPlayerPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SpawnPlayerPlugin, ScorePlugin, SpawnMainSpritePlugin));
    }
}
