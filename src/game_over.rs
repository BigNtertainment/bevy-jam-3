use bevy::prelude::*;
use bevy_console::PrintConsoleLine;

use crate::{cleanup::cleanup, GameState};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_go_ui.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(cleanup::<GameOverUI>.in_schedule(OnExit(GameState::GameOver)));
    }
}

#[derive(Component)]
pub struct GameOverUI;

pub fn setup_go_ui(mut commands: Commands, mut console_line: EventWriter<PrintConsoleLine>) {
    console_line.send(PrintConsoleLine::new("player died".into()));
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: UiRect::all(Val::Px(20.0)),
                ..Default::default()
            },
            background_color: Color::Rgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 0.4 }.into(),
            ..Default::default()
        })
        .insert(Name::new("GameOverUI"))
        .insert(GameOverUI);
}
