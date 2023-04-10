use bevy::prelude::*;

use crate::{
    loading::{FontAssets, TextureAssets},
    unit::Health,
};

use super::{inventory::Inventory, Player};

#[derive(Component, Reflect, Clone, Debug, Default, PartialEq)]
#[reflect(Component)]
pub struct PlayerUI;

#[derive(Component, Reflect, Clone, Debug, Default, PartialEq)]
#[reflect(Component)]
pub struct HealthUI;

#[derive(Component, Reflect, Clone, Debug, Default, PartialEq)]
#[reflect(Component)]
pub struct InventorySlotUI {
    pub index: usize,
}

#[derive(Component, Reflect, Clone, Debug, Default, PartialEq)]
#[reflect(Component)]
pub struct EffectsUI;

pub fn setup_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
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
                .insert(Name::new("Health Container"))
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

            parent
                .spawn(NodeBundle {
                    style: Style {
                        margin: UiRect::top(Val::Px(20.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Name::new("Inventory Container"))
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: Color::BLACK.into(),
                            ..Default::default()
                        })
                        .insert(Name::new("Inventory Slot #1 Border"))
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::all(Val::Px(3.0)),
                                        ..Default::default()
                                    },
                                    background_color: Color::BEIGE.into(),
                                    ..Default::default()
                                })
                                .insert(Name::new("Inventory Slot #1 Container"))
                                .with_children(|parent| {
                                    parent
                                        .spawn(ImageBundle {
                                            style: Style {
                                                size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                                                margin: UiRect::all(Val::Px(5.0)),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(InventorySlotUI { index: 0 })
                                        .insert(Name::new("Inventory Slot #1"));
                                });
                        });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::left(Val::Px(20.0)),
                                ..Default::default()
                            },
                            background_color: Color::BLACK.into(),
                            ..Default::default()
                        })
                        .insert(Name::new("Inventory Slot #2 Border"))
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::all(Val::Px(3.0)),
                                        ..Default::default()
                                    },
                                    background_color: Color::BEIGE.into(),
                                    ..Default::default()
                                })
                                .insert(Name::new("Inventory Slot #2 Container"))
                                .with_children(|parent| {
                                    parent
                                        .spawn(ImageBundle {
                                            style: Style {
                                                size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                                                margin: UiRect::all(Val::Px(5.0)),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(InventorySlotUI { index: 1 })
                                        .insert(Name::new("Inventory Slot #2"));
                                });
                        });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::left(Val::Px(20.0)),
                                ..Default::default()
                            },
                            background_color: Color::BLACK.into(),
                            ..Default::default()
                        })
                        .insert(Name::new("Inventory Slot #3 Border"))
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::all(Val::Px(3.0)),
                                        ..Default::default()
                                    },
                                    background_color: Color::BEIGE.into(),
                                    ..Default::default()
                                })
                                .insert(Name::new("Inventory Slot #3 Container"))
                                .with_children(|parent| {
                                    parent
                                        .spawn(ImageBundle {
                                            style: Style {
                                                size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                                                margin: UiRect::all(Val::Px(5.0)),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(InventorySlotUI { index: 2 })
                                        .insert(Name::new("Inventory Slot #3"));
                                });
                        });
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        margin: UiRect::top(Val::Px(20.0)),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(EffectsUI)
                .insert(Name::new("Effect Bars Container"));
        });
}

pub fn update_health_ui(
    player_query: Query<&Health, With<Player>>,
    mut health_ui_query: Query<&mut Text, With<HealthUI>>,
) {
    let player_health = player_query.single();

    let mut health_ui = health_ui_query.single_mut();
    health_ui.sections[0].value = format!(
        "{:.0}/{}",
        player_health.get_health(),
        player_health.get_max_health()
    );
}

pub fn update_inventory_ui(
    player_query: Query<&Inventory, With<Player>>,
    mut inventory_ui_query: Query<(&mut UiImage, &InventorySlotUI)>,
    textures: Res<TextureAssets>,
) {
    let player_inventory = player_query.single();

    for (mut inventory_ui, inventory_slot_ui) in inventory_ui_query.iter_mut() {
        let item = player_inventory.get_pill(inventory_slot_ui.index);

        if let Some(item) = item {
            inventory_ui.texture = item.get_texture(&textures);
        } else {
            inventory_ui.texture = default();
        }
    }
}
