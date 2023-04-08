use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody, Sensor};
use bevy_spritesheet_animation::{
    animation::{Animation, AnimationBounds},
    animation_graph::{AnimationTransitionCondition, AnimationTransitionMode},
    animation_manager::AnimationManager,
};

use crate::{
    cleanup::cleanup,
    loading::TextureAssets,
    unit::{Direction, Movement},
    GameState, WorldState,
};

use self::{
    attack::{EnemyAttackPlugin, EnemyAttackTimer},
    movement::{EnemyMovementPlugin, EnemyMovementTarget, EnemyMovementType},
    sight::EnemySightPlugin,
};

mod attack;
mod movement;
mod sight;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EnemyState>()
            .add_plugin(EnemyMovementPlugin)
            .add_plugin(EnemySightPlugin)
            .add_plugin(EnemyAttackPlugin)
            .add_system(debug_spawn.in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (update_sprites, handle_stunned_enemies).in_set(OnUpdate(GameState::Playing)),
            )
            .add_system(cleanup::<EnemyState>.in_schedule(OnExit(WorldState::Yes)));
    }
}

#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub enum EnemyState {
    #[default]
    Idle,
    Alert {
        target: Vec2,
    },
    Stun {
        timer: Timer,
    },
}

#[derive(Bundle)]
pub struct EnemyBundle {
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    movement: Movement,
    direction: Direction,
    rigidbody: RigidBody,
    collider: Collider,
    sensor: Sensor,
    state: EnemyState,
    movement_type: EnemyMovementType,
    movement_target: EnemyMovementTarget,
    animation_manager: AnimationManager,
    attack_timer: EnemyAttackTimer,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        let mut animation_manager = AnimationManager::new(
            vec![
                // Idle
                Animation::new(AnimationBounds::new(0, 0), Duration::from_millis(500)),
                // Walking
                Animation::new(AnimationBounds::new(0, 19), Duration::from_millis(80)),
                // Stun
                Animation::new(AnimationBounds::new(20, 21), Duration::from_millis(350)),
                // Shooting
                Animation::new(AnimationBounds::new(22, 52), Duration::from_millis(80)),
            ],
            0,
        );

        animation_manager.add_state("walk".to_string(), false);
        animation_manager.add_state("stun".to_string(), false);
        animation_manager.add_state("shoot".to_string(), false);

        animation_manager.add_graph_edge(
            0,
            1,
            AnimationTransitionCondition::new(Box::new(|state| state["walk"]))
                .with_mode(AnimationTransitionMode::Immediate),
        );
        animation_manager.add_graph_edge(
            1,
            0,
            AnimationTransitionCondition::new(Box::new(|state| !state["walk"]))
                .with_mode(AnimationTransitionMode::Immediate),
        );
        animation_manager.add_graph_edge(
            1,
            1,
            AnimationTransitionCondition::new(Box::new(|state| state["walk"])),
        );

        animation_manager.add_graph_edge(
            0,
            2,
            AnimationTransitionCondition::new(Box::new(|state| state["stun"]))
                .with_mode(AnimationTransitionMode::Immediate),
        );
        animation_manager.add_graph_edge(
            1,
            2,
            AnimationTransitionCondition::new(Box::new(|state| state["stun"]))
                .with_mode(AnimationTransitionMode::Immediate),
        );
        animation_manager.add_graph_edge(
            2,
            2,
            AnimationTransitionCondition::new(Box::new(|state| state["stun"])),
        );
        animation_manager.add_graph_edge(
            2,
            0,
            AnimationTransitionCondition::new(Box::new(|state| !state["stun"]))
                .with_mode(AnimationTransitionMode::Immediate),
        );

        animation_manager.add_graph_edge(
            0,
            3,
            AnimationTransitionCondition::new(Box::new(|state| state["shoot"]))
                .with_mode(AnimationTransitionMode::Immediate),
        );
        animation_manager.add_graph_edge(
            1,
            3,
            AnimationTransitionCondition::new(Box::new(|state| state["shoot"]))
                .with_mode(AnimationTransitionMode::Immediate),
        );
        animation_manager.add_graph_edge(
            2,
            0,
            AnimationTransitionCondition::new(Box::new(|state| !state["shoot"])),
        );

        Self {
            sprite_sheet_bundle: SpriteSheetBundle::default(),
            movement: Movement {
                speed: 100.,
                running_speed: 215.,
            },
            direction: Direction::default(),
            rigidbody: RigidBody::KinematicPositionBased,
            collider: Collider::cuboid(32., 128.),
            sensor: Sensor,
            state: EnemyState::default(),
            movement_type: EnemyMovementType::Static { target: Vec2::ZERO },
            movement_target: EnemyMovementTarget::default(),
            animation_manager,
            attack_timer: EnemyAttackTimer(Timer::from_seconds(1., TimerMode::Repeating)),
        }
    }
}

/// debug_spawn spawns test enemy somwehere on the map.
fn debug_spawn(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(EnemyBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                transform: Transform::from_xyz(-100., 50., 0.)
                    .with_scale(Vec2::splat(0.5).extend(1.)),
                texture_atlas: textures.enemy_down.clone(),
                sprite: TextureAtlasSprite::new(0),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Enemy #1"));

    commands
        .spawn(EnemyBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                transform: Transform::from_xyz(200., 250., 0.)
                    .with_scale(Vec2::splat(0.5).extend(1.)),
                texture_atlas: textures.enemy_down.clone(),
                sprite: TextureAtlasSprite::new(0),
                ..default()
            },
            movement_type: EnemyMovementType::AlongPath {
                path: vec![
                    Vec2::new(200., 250.),
                    Vec2::new(200., -250.),
                    Vec2::new(100., -250.),
                    Vec2::new(100., 250.),
                ],
                current: 0,
            },
            ..default()
        })
        .insert(Name::new("Enemy #2"));

    commands
        .spawn(EnemyBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                transform: Transform::from_xyz(150., 150., 0.)
                    .with_scale(Vec2::splat(0.5).extend(1.)),
                texture_atlas: textures.enemy_down.clone(),
                sprite: TextureAtlasSprite::new(0),
                ..default()
            },
            movement_type: EnemyMovementType::GuardArea {
                area: Rect::new(0., 100., 300., 300.),
                current: Vec2::new(150., 150.),
                wait_timer: Timer::from_seconds(3., TimerMode::Repeating),
            },
            movement: Movement {
                speed: 50.,
                running_speed: 215.,
            },
            ..default()
        })
        .insert(Name::new("Enemy #3"));
}

fn update_sprites(
    mut enemy_query: Query<
        (&mut Handle<TextureAtlas>, &Direction),
        (With<EnemyState>, Changed<Direction>),
    >,
    textures: Res<TextureAssets>,
) {
    for (mut texture, direction) in enemy_query.iter_mut() {
        *texture = match direction {
            Direction::Up => textures.enemy_up.clone(),
            Direction::Down => textures.enemy_down.clone(),
            Direction::Left => textures.enemy_left.clone(),
            Direction::Right => textures.enemy_right.clone(),
        }
    }
}

fn handle_stunned_enemies(mut enemy_query: Query<(&mut EnemyState, &mut AnimationManager)>, time: Res<Time>) {
    for (mut enemy_state, mut animation_manager) in enemy_query.iter_mut() {
        if let EnemyState::Stun { timer } = enemy_state.as_mut() {
            timer.tick(time.delta());

            animation_manager.set_state("stun".to_string(), true).unwrap();

            if timer.finished() {
                *enemy_state = EnemyState::Idle;
                animation_manager.set_state("stun".to_string(), false).unwrap();
            }
        }
    }
}
