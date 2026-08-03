use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0));
        app.add_systems(Startup, spawn_score)
            .add_systems(Update, set_score)
            .add_systems(Update, increment_score);
    }
}

#[derive(Resource)]
struct Score(pub i32);

#[derive(Component)]
struct ScoreText;

fn spawn_score(mut commands: Commands) {
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            ..Default::default()
        })
        .with_children(|score| {
            score.spawn((
                Text::new("0"),
                TextFont {
                    font_size: FontSize::Px(60.),
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                ScoreText,
            ));
        });
}

fn set_score(score: Res<Score>, mut score_text: Single<&mut Text, With<ScoreText>>) {
    score_text.0 = format!("{}", score.0);
}

fn increment_score(mut score: ResMut<Score>) {
    score.0 += 1;
}
