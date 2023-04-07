use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody, Sensor};

use crate::{
    cleanup::cleanup,
    loading::TextureAssets,
    unit::{Direction, Movement},
    GameState,
};

use self::{
    movement::{EnemyMovementPlugin, EnemyMovementTarget, EnemyMovementType},
    sight::EnemySightPlugin,
};

mod movement;
mod sight;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EnemyMovementPlugin)
            .add_plugin(EnemySightPlugin)
            .add_system(debug_spawn.in_schedule(OnEnter(GameState::Playing)))
            .add_system(update_sprites.in_set(OnUpdate(GameState::Playing)))
            .add_system(cleanup::<EnemyState>.in_schedule(OnExit(GameState::Playing)));
    }
}

#[derive(Component, Debug, Clone, Default, PartialEq, Reflect)]
#[reflect(Component)]
pub enum EnemyState {
    #[default]
    Idle,
    Alert {
        target: Vec2,
    },
    Attacking,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    movement: Movement,
    direction: Direction,
    rigidbody: RigidBody,
    collider: Collider,
    sensor: Sensor,
    state: EnemyState,
    movement_type: EnemyMovementType,
    movement_target: EnemyMovementTarget,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteBundle::default(),
            movement: Movement { speed: 100. },
            direction: Direction::default(),
            rigidbody: RigidBody::KinematicPositionBased,
            collider: Collider::cuboid(32., 128.),
            sensor: Sensor,
            state: EnemyState::default(),
            movement_type: EnemyMovementType::Static { target: Vec2::ZERO },
            movement_target: EnemyMovementTarget::default(),
        }
    }
}

/// debug_spawn spawns test enemy somwehere on the map.
fn debug_spawn(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(EnemyBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_xyz(-100., 50., 0.)
                    .with_scale(Vec2::splat(0.5).extend(1.)),
                texture: textures.enemy_down.clone(),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Enemy #1"));

    commands
        .spawn(EnemyBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_xyz(200., 250., 0.)
                    .with_scale(Vec2::splat(0.5).extend(1.)),
                texture: textures.enemy_down.clone(),
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
            sprite_bundle: SpriteBundle {
                transform: Transform::from_xyz(150., 150., 0.)
                    .with_scale(Vec2::splat(0.5).extend(1.)),
                texture: textures.enemy_down.clone(),
                ..default()
            },
            movement_type: EnemyMovementType::GuardArea {
                area: Rect::new(0., 100., 300., 300.),
                current: Vec2::new(150., 150.),
                wait_timer: Timer::from_seconds(3., TimerMode::Repeating),
            },
            movement: Movement { speed: 50. },
            ..default()
        })
        .insert(Name::new("Enemy #3"));
}

fn update_sprites(
    mut enemy_query: Query<
        (&mut Handle<Image>, &Direction),
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
