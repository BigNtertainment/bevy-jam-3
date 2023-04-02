use bevy::prelude::*;

use crate::{loading::FontAssets, unit::Health};

use super::Player;

#[derive(Component)]
pub struct PlayerUI;

#[derive(Component)]
pub struct HealthUI;

pub fn setup_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
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
        .insert(Name::new("UI"))
        .insert(PlayerUI)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Name::new("Heath Container"))
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle::from_section(
                            "0/0",
                            TextStyle {
                                font: font_assets.space_grotesk.clone(),
                                font_size: 32.0,
                                color: Color::WHITE, //TODO: Better colour
                            },
                        ))
                        .insert(HealthUI)
                        .insert(Name::new("Health UI"));
                });
        });
}

pub fn update_ui(
    player_query: Query<&Health, With<Player>>,
    mut health_ui_query: Query<&mut Text, With<HealthUI>>,
) {
    let player_health = player_query.single();

    let mut health_ui = health_ui_query.single_mut();
    health_ui.sections[0].value = format!(
        "{}/{}",
        player_health.get_health().to_string(),
        player_health.get_max_health().to_string()
    );
}
