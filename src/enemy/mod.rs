use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::{FieldValue, LayerInstance, LdtkEntity, TilesetDefinition},
    utils::ldtk_pixel_coords_to_translation_pivoted,
    EntityInstance,
};
use bevy_rapier2d::prelude::{Collider, RigidBody, Sensor};
use bevy_spritesheet_animation::animation_manager::AnimationManager;

use crate::{
    cleanup::cleanup,
    loading::TextureAssets,
    unit::{Direction, Movement},
    WorldState,
};

use self::{
    animation::enemy_animation_manager,
    attack::{EnemyAttackPlugin, EnemyAttackTimer},
    movement::{EnemyMovementPlugin, EnemyMovementTarget, EnemyMovementType},
    sight::EnemySightPlugin,
};

mod animation;
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
            .add_systems(
                (
                    update_sprites,
                    update_z_index,
                    adjust_enemy_scale,
                    handle_stunned_enemies,
                )
                    .in_set(OnUpdate(WorldState::Yes)),
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

pub const ENEMY_COLLIDER_WIDTH: f32 = 32.;
pub const ENEMY_COLLIDER_HEIGHT: f32 = 120.;

impl Default for EnemyBundle {
    fn default() -> Self {
        let animation_manager = enemy_animation_manager();

        Self {
            sprite_sheet_bundle: SpriteSheetBundle::default(),
            movement: Movement {
                speed: 100.,
                running_speed: 215.,
            },
            direction: Direction::default(),
            rigidbody: RigidBody::KinematicPositionBased,
            collider: Collider::cuboid(ENEMY_COLLIDER_WIDTH, ENEMY_COLLIDER_HEIGHT),
            sensor: Sensor,
            state: EnemyState::default(),
            movement_type: EnemyMovementType::Static { target: Vec2::ZERO },
            movement_target: EnemyMovementTarget::default(),
            animation_manager,
            attack_timer: EnemyAttackTimer(Timer::from_seconds(1., TimerMode::Repeating)),
        }
    }
}

impl LdtkEntity for EnemyBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        let mut movement_type_str = "".to_string();
        let position = ldtk_pixel_coords_to_translation_pivoted(
            entity_instance.px,
            layer_instance.c_hei * layer_instance.grid_size,
            IVec2::new(entity_instance.width, entity_instance.height),
            entity_instance.pivot,
        );

        let mut path = vec![position];
        let mut area_corner_1 = None;
        let mut area_corner_2 = None;

        for field in &entity_instance.field_instances {
            match field.identifier.as_str() {
                "Enemy_Type" => match &field.value {
                    FieldValue::Enum(value) => {
                        movement_type_str = value.clone().expect("Movement type is null");
                    }
                    other => panic!("Unknown movement type: {:?}", other),
                },
                "Path" => match &field.value {
                    FieldValue::Points(value) => {
                        let path_points = value
                            .into_iter()
                            .map(|point| {
                                let point = point.expect("Empty point in an enemy path!");

                                ldtk_pixel_coords_to_translation_pivoted(
                                    point * layer_instance.grid_size,
                                    layer_instance.c_hei * layer_instance.grid_size,
                                    IVec2::new(entity_instance.width, entity_instance.height),
                                    entity_instance.pivot,
                                )
                            })
                            .collect::<Vec<_>>();

                        for point in path_points {
                            path.push(point);
                        }
                    }
                    other => panic!("Unknown movement type: {:?}", other),
                },
                "GuardedAreaCorner1" => match &field.value {
                    FieldValue::Point(value) => {
                        area_corner_1 = value.map(|point| {
                            ldtk_pixel_coords_to_translation_pivoted(
                                point * layer_instance.grid_size,
                                layer_instance.c_hei * layer_instance.grid_size,
                                IVec2::new(entity_instance.width, entity_instance.height),
                                entity_instance.pivot,
                            )
                        });
                    }
                    other => panic!("Unknown movement type: {:?}", other),
                },
                "GuardedAreaCorner2" => match &field.value {
                    FieldValue::Point(value) => {
                        area_corner_2 = value.map(|point| {
                            ldtk_pixel_coords_to_translation_pivoted(
                                point * layer_instance.grid_size,
                                layer_instance.c_hei * layer_instance.grid_size,
                                IVec2::new(entity_instance.width, entity_instance.height),
                                entity_instance.pivot,
                            )
                        });
                    }
                    other => panic!("Unknown movement type: {:?}", other),
                },
                other => panic!("Unknown enemy field: {}", other),
            }
        }

        let movement_type = match movement_type_str.as_str() {
            "Static" => EnemyMovementType::Static { target: position },
            "AlongPath" => {
                println!("{:?}", path);
                EnemyMovementType::AlongPath { path, current: 0 }
            }
            "GuardArea" => {
                let corner_1 = area_corner_1.expect("Missing enemy area corner 1");
                let corner_2 = area_corner_2.expect("Missing enemy area corner 2");

                EnemyMovementType::GuardArea {
                    area: Rect::from_corners(corner_1, corner_2),
                    current: corner_1,
                    wait_timer: Timer::from_seconds(3., TimerMode::Repeating),
                }
            }
            other => panic!("Unknown enemy movement type: {}", other),
        };

        Self {
            movement_type,
            ..default()
        }
    }
}

fn adjust_enemy_scale(mut enemy_query: Query<&mut Transform, Added<EnemyState>>) {
    for mut transform in enemy_query.iter_mut() {
        transform.scale = Vec2::splat(0.5).extend(1.0);
    }
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

fn update_z_index(mut enemy_query: Query<(&mut Transform, &Direction), With<EnemyState>>) {
    for (mut transform, direction) in enemy_query.iter_mut() {
        transform.translation.z = match direction {
            Direction::Up => 6.,
            _ => 4.,
        }
    }
}

fn handle_stunned_enemies(
    mut enemy_query: Query<(&mut EnemyState, &mut AnimationManager)>,
    time: Res<Time>,
) {
    for (mut enemy_state, mut animation_manager) in enemy_query.iter_mut() {
        if let EnemyState::Stun { timer } = enemy_state.as_mut() {
            timer.tick(time.delta());

            animation_manager
                .set_state("stun".to_string(), true)
                .unwrap();

            if timer.finished() {
                *enemy_state = EnemyState::Idle;
                animation_manager
                    .set_state("stun".to_string(), false)
                    .unwrap();
            }
        }
    }
}
