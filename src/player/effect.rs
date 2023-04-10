use std::marker::PhantomData;

use bevy::{ecs::schedule::SystemConfigs, prelude::*};
use bevy_kira_audio::{Audio, AudioControl};
use drug_test_proc_macros::Temporary;

use crate::{
    enemy::EnemyState,
    loading::{AudioAssets, FontAssets},
    pill::{Pill, PillEffect},
    unit::Health,
    GameState,
};

use super::{ui::EffectsUI, Player};

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MovementBoost>()
            .register_type::<Invisibility>()
            .register_type::<Invincibility>()
            .register_type::<Dizziness>()
            .register_type::<Vulnerability>()
            .add_systems(effect_systems::<MovementBoost>())
            .add_systems(effect_systems::<Invisibility>())
            .add_systems(effect_systems::<Invincibility>())
            .add_systems(effect_systems::<Dizziness>())
            .add_systems(effect_systems::<Vulnerability>())
            .add_system(invisibility_vfx.in_set(OnUpdate(GameState::Playing)))
            .add_system(reset_invisibility_vfx.in_set(OnUpdate(GameState::Playing)));
    }
}

pub fn execute_pill_effects(
    In(pills): In<Vec<Pill>>,
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Health, &Transform), With<Player>>,
    mut enemy_query: Query<(&mut EnemyState, &Transform)>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    let (player_entity, mut player_health, player_transform) = player_query.single_mut();

    for pill in pills {
        for effect in [pill.main_effect, pill.side_effect] {
            match effect {
                PillEffect::Heal { amount } => {
                    player_health.heal(amount);
                }
                PillEffect::Speed { amount, duration } => {
                    commands.entity(player_entity).insert(MovementBoost {
                        timer: Timer::new(duration, TimerMode::Once),
                        multiplier: amount,
                    });
                }
                PillEffect::ToxicFart => {
                    for (mut enemy_state, enemy_transform) in enemy_query.iter_mut() {
                        let distance = enemy_transform
                            .translation
                            .truncate()
                            .distance(player_transform.translation.truncate());

                        if distance < 150.0 {
                            *enemy_state = EnemyState::Stun {
                                timer: Timer::from_seconds(5., TimerMode::Once),
                            };
                        }
                    }

                    audio.play(audio_assets.fart.clone());
                }
                PillEffect::Invisibility { duration } => {
                    commands.entity(player_entity).insert(Invisibility {
                        timer: Timer::new(duration, TimerMode::Once),
                    });
                }
                PillEffect::Invincibility { duration } => {
                    commands.entity(player_entity).insert(Invincibility {
                        timer: Timer::new(duration, TimerMode::Once),
                    });
                }
                PillEffect::Vulnerability { amount, duration } => {
                    commands.entity(player_entity).insert(Vulnerability {
                        amount,
                        timer: Timer::new(duration, TimerMode::Once),
                    });
                }
                PillEffect::Dizziness { duration } => {
                    commands.entity(player_entity).insert(Dizziness {
                        timer: Timer::new(duration, TimerMode::Once),
                    });
                }
                PillEffect::Sneeze => {
                    for (mut enemy_state, enemy_transform) in enemy_query.iter_mut() {
                        let distance = enemy_transform
                            .translation
                            .truncate()
                            .distance(player_transform.translation.truncate());

                        if distance < 450.0 && !matches!(*enemy_state, EnemyState::Stun { .. }) {
                            *enemy_state = EnemyState::Alert {
                                target: player_transform.translation.truncate(),
                            };
                        }
                    }

                    audio.play(audio_assets.sneeze.clone());
                }
            }
        }
    }
}

pub trait Temporary {
    fn get_timer(&mut self) -> &mut Timer;
}

pub trait EffectVisuals {
    fn get_color(&self) -> Color;
    fn get_name(&self) -> String;
}

fn effect_systems<Effect: Temporary + Component + EffectVisuals>() -> SystemConfigs {
    (
        update_effect::<Effect>,
        setup_effect_ui::<Effect>,
        update_effect_ui::<Effect>,
        remove_effect_ui::<Effect>,
    )
        .in_set(OnUpdate(GameState::Playing))
}

#[derive(Reflect, Component, Clone, Default, Debug, Temporary)]
#[reflect(Component)]
pub struct MovementBoost {
    pub multiplier: f32,
    pub timer: Timer,
}

impl EffectVisuals for MovementBoost {
    fn get_color(&self) -> Color {
        if self.multiplier > 1.0 {
            Color::hex("eac516").unwrap()
        } else {
            Color::hex("daa11d").unwrap()
        }
    }

    fn get_name(&self) -> String {
        if self.multiplier > 1.0 {
            "Speed Boost".to_string()
        } else {
            "Speed Debuff".to_string()
        }
    }
}

#[derive(Reflect, Component, Clone, Default, Debug, Temporary)]
#[reflect(Component)]
pub struct Invisibility {
    pub timer: Timer,
}

impl EffectVisuals for Invisibility {
    fn get_color(&self) -> Color {
        Color::hex("00ecc7").unwrap()
    }

    fn get_name(&self) -> String {
        "Invisibility".to_string()
    }
}

#[derive(Reflect, Component, Clone, Default, Debug, Temporary)]
#[reflect(Component)]
pub struct Invincibility {
    pub timer: Timer,
}

impl EffectVisuals for Invincibility {
    fn get_color(&self) -> Color {
        Color::hex("dfc1ff").unwrap()
    }

    fn get_name(&self) -> String {
        "Invincibility".to_string()
    }
}

#[derive(Reflect, Component, Clone, Default, Debug, Temporary)]
#[reflect(Component)]
pub struct Dizziness {
    pub timer: Timer,
}

impl EffectVisuals for Dizziness {
    fn get_color(&self) -> Color {
        Color::hsl(self.timer.percent() * 360., 1.0, 0.5)
    }

    fn get_name(&self) -> String {
        "Dizziness".to_string()
    }
}

#[derive(Reflect, Component, Clone, Default, Debug, Temporary)]
#[reflect(Component)]
pub struct Vulnerability {
    pub amount: f32,
    pub timer: Timer,
}

impl EffectVisuals for Vulnerability {
    fn get_color(&self) -> Color {
        Color::hex("fc620a").unwrap()
    }

    fn get_name(&self) -> String {
        "Vulnerability".to_string()
    }
}

fn update_effect<Effect: Temporary + Component>(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Effect)>,
    time: Res<Time>,
) {
    for (entity, mut effect) in query.iter_mut() {
        if effect.get_timer().tick(time.delta()).just_finished() {
            commands.entity(entity).remove::<Effect>();
        }
    }
}

#[derive(Component, Copy, Clone, Debug, Default)]
struct EffectBarContainerMarker<Effect: Temporary + Component + EffectVisuals> {
    _effect: PhantomData<Effect>,
}

impl<Effect: Temporary + Component + EffectVisuals> EffectBarContainerMarker<Effect> {
    fn new() -> Self {
        Self {
            _effect: PhantomData,
        }
    }
}

#[derive(Component, Copy, Clone, Debug, Default)]
struct EffectBarMarker<Effect: Temporary + Component + EffectVisuals> {
    _effect: PhantomData<Effect>,
}

impl<Effect: Temporary + Component + EffectVisuals> EffectBarMarker<Effect> {
    fn new() -> Self {
        Self {
            _effect: PhantomData,
        }
    }
}

fn setup_effect_ui<Effect: Temporary + Component + EffectVisuals>(
    mut commands: Commands,
    effect_query: Query<&Effect, (Added<Effect>, With<Player>)>,
    ui_query: Query<Entity, With<EffectsUI>>,
    font_assets: Res<FontAssets>,
) {
    let ui_parent = ui_query.single();

    for effect in effect_query.iter() {
        let color = effect.get_color();
        let name = effect.get_name();

        commands.entity(ui_parent).with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Px(200.0), Val::Px(40.0)),
                        padding: UiRect::all(Val::Px(7.0)),
                        margin: UiRect::bottom(Val::Px(10.)),
                        ..Default::default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                })
                .insert(EffectBarContainerMarker::<Effect>::new())
                .insert(Name::new("Effect Bar Border"))
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(5.)),
                                ..default()
                            },
                            background_color: color.into(),
                            ..default()
                        })
                        .insert(EffectBarMarker::<Effect>::new())
                        .insert(Name::new("Effect Bar"))
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle {
                                    text: Text::from_section(
                                        name,
                                        TextStyle {
                                            font: font_assets.fira_sans.clone(),
                                            font_size: 20.0,
                                            color: Color::BLACK,
                                        },
                                    ),
                                    ..default()
                                })
                                .insert(Name::new("Effect Name"));
                        });
                });
        });
    }
}

fn update_effect_ui<Effect: Temporary + Component + EffectVisuals>(
    mut effect_query: Query<&mut Effect, With<Player>>,
    mut effect_bar_query: Query<(&mut Style, &mut BackgroundColor), With<EffectBarMarker<Effect>>>,
) {
    for mut effect in effect_query.iter_mut() {
        for (mut style, mut bg_color) in effect_bar_query.iter_mut() {
            style.size.width = Val::Percent((1. - effect.get_timer().percent()) * 100.0);
            bg_color.0 = effect.get_color().into();
        }
    }
}

fn remove_effect_ui<Effect: Temporary + Component + EffectVisuals>(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut effect_bar_container_query: Query<Entity, With<EffectBarContainerMarker<Effect>>>,
    mut effect_removals: RemovedComponents<Effect>,
) {
    let player = player_query.single();

    for effect_bar_entity in effect_bar_container_query.iter_mut() {
        for entity in effect_removals.iter() {
            if entity == player {
                commands.entity(effect_bar_entity).despawn_recursive();
            }
        }
    }
}

fn invisibility_vfx(mut invisible_query: Query<&mut Sprite, Added<Invisibility>>) {
    for mut sprite in invisible_query.iter_mut() {
        sprite.color.set_a(0.5);
    }
}

fn reset_invisibility_vfx(
    mut invisible_query: Query<&mut Sprite>,
    mut invisibility_removals: RemovedComponents<Invisibility>,
) {
    for entity in invisibility_removals.iter() {
        if let Ok(mut sprite) = invisible_query.get_mut(entity) {
            sprite.color.set_a(1.0);
        }
    }
}
