use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background)
            .add_systems(Update, resize_background);
    }
}

#[derive(Component)]
struct Background;

fn spawn_background(mut commands: Commands, assets_server: ResMut<AssetServer>) {
    commands.spawn((
        Sprite::from_image(assets_server.load("background.png")),
        Transform::from_xyz(0., 0., -10.),
        Background,
    ));
}

fn resize_background(
    mut background: Single<&mut Sprite, With<Background>>,
    window: Single<&Window>,
) {
    background.custom_size = Some(Vec2::new(window.width(), window.height()));
}
