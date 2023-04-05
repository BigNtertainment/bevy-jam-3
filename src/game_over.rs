use bevy::prelude::*;
use bevy_console::PrintConsoleLine;

use crate::{cleanup::cleanup, GameState, loading::FontAssets, WorldState};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_go_ui.in_schedule(OnEnter(GameState::GameOver)))
        .add_system(click_main_menu_button.in_set(OnUpdate(GameState::GameOver)))
        .add_system(cleanup::<GameOverUI>.in_schedule(OnExit(GameState::GameOver)));
    }
}

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct MainMenuButton;

pub fn setup_go_ui(
    mut commands: Commands,
    mut console_line: EventWriter<PrintConsoleLine>,
    font_assets: Res<FontAssets>,
) {
    console_line.send(PrintConsoleLine::new("player died".into()));
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: UiRect::all(Val::Px(20.0)),
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(Name::new("GameOverUI"))
        .insert(GameOverUI)
        .with_children(|parent| {
            parent
            .spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..Default::default()
            })
            .insert(Name::new("Main Menu Button"))
            .insert(MainMenuButton)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Main Menu",
                    TextStyle {
                        font: font_assets.space_grotesk.clone(),
                        font_size: 40.0,
                        color: Color::WHITE.into(),
                    },
                ));
            });
        });
}

fn click_main_menu_button(
    mut game_state: ResMut<NextState<GameState>>,
    mut world_state: ResMut<NextState<WorldState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                game_state.set(GameState::Menu);
                world_state.set(WorldState::No);
            }
            Interaction::Hovered => {
                *color = Color::Rgba { red: 0.8, green: 0.8, blue: 0.8, alpha: 0.8 }.into();
            }
            Interaction::None => {
                *color = Color::BLACK.into();
            }
        }
    }
}