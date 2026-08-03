use bevy::prelude::*;

pub struct SpawnPlayerPlugin;

impl Plugin for SpawnPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default(), Transform::from_xyz(0., 0., 0.)));
}
