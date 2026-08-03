use bevy::prelude::*;

use crate::GameState;

pub struct SpawnMainSpritePlugin;

#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
pub struct SpawnMainSpriteSet;

impl Plugin for SpawnMainSpritePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            SpawnMainSpriteSet.run_if(in_state(GameState::Playing)),
        );
        app.add_systems(OnEnter(GameState::Playing), spawn_sprite)
            .add_systems(Update, out_of_bounds.in_set(SpawnMainSpriteSet))
            .add_systems(Update, move_sprite.in_set(SpawnMainSpriteSet));
    }
}

#[derive(Component)]
struct MainSprite;

fn spawn_sprite(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: assets_server.load("main_sprite.png"),
            custom_size: Some(Vec2::new(100., 100.)),
            ..Default::default()
        },
        Transform::from_xyz(0., 0., 0.),
        MainSprite,
    ));
}

fn move_sprite(
    key: ResMut<ButtonInput<KeyCode>>,
    mut sprite: Single<&mut Transform, With<MainSprite>>,
) {
    if key.pressed(KeyCode::ArrowRight) {
        sprite.translation.x += 5.;
    }

    if key.pressed(KeyCode::ArrowLeft) {
        sprite.translation.x -= 5.;
    }

    if key.pressed(KeyCode::ArrowUp) {
        sprite.translation.y += 5.;
    }

    if key.pressed(KeyCode::ArrowDown) {
        sprite.translation.y -= 5.;
    }
}

fn out_of_bounds(sprite_trans: Single<&Transform, With<MainSprite>>, window: Single<&Window>) {
    if sprite_trans.translation.x == (window.width() / 2. - 50.) {
        println!("right")
    }

    if sprite_trans.translation.x == (-window.width() / 2. + 50.) {
        println!("left")
    }

    if sprite_trans.translation.y == (window.height() / 2. - 50.) {
        println!("top")
    }

    if sprite_trans.translation.y == (-window.height() / 2. + 50.) {
        println!("bottom")
    }
}
