use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, QueryFilter, RapierContext, RigidBody};
use bevy_spritesheet_animation::{
    animation::{Animation, AnimationBounds},
    animation_graph::{AnimationTransitionCondition, AnimationTransitionMode},
    animation_manager::{transition_animations, AnimationManager},
};

use crate::{
    actions::{Actions, BurstActions},
    cleanup::cleanup,
    enemy::EnemyState,
    loading::TextureAssets,
    pill::Pill,
    unit::{Direction, Euler, Health, Movement},
    GameState, WorldState,
};

use self::{
    effect::{execute_pill_effects, Dizziness, EffectPlugin, MovementBoost},
    inventory::Inventory,
    ui::{setup_ui, update_health_ui, update_inventory_ui, HealthUI, InventorySlotUI, PlayerUI},
};

pub mod effect;
mod inventory;
mod ui;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .register_type::<PunchTimer>()
            .register_type::<PlayerUI>()
            .register_type::<HealthUI>()
            .register_type::<InventorySlotUI>()
            .add_plugin(EffectPlugin)
            .add_systems((setup_player, setup_ui).in_schedule(OnEnter(WorldState::Yes)))
            .add_systems(
                (
                    player_movement,
                    punch_enemies.after(transition_animations),
                    pick_up_pills,
                    consume_pills.pipe(execute_pill_effects),
                    update_sprite,
                    update_health_ui,
                    update_inventory_ui,
                    damage_yourself,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            )
            .add_system(spawn_player_body.in_set(OnUpdate(WorldState::Yes)))
            .add_systems(
                (cleanup::<Player>, cleanup::<PlayerBody>).in_schedule(OnEnter(WorldState::No)),
            )
            .add_system(cleanup::<PlayerUI>.in_schedule(OnExit(GameState::Playing)));
    }
}

#[derive(Reflect, Component, Copy, Clone, Default, Debug, PartialEq, Eq)]
#[reflect(Component)]
pub struct Player;

#[derive(Reflect, Component, Copy, Clone, Default, Debug, PartialEq, Eq)]
#[reflect(Component)]
pub struct PlayerBody;

#[derive(Reflect, Component, Clone, Default, Debug, Deref, DerefMut)]
#[reflect(Component)]
pub struct PunchTimer(pub Timer);

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    animation_manager: AnimationManager,
    // TODO: make an issue in rapier so they register their types
    rigidbody: RigidBody,
    collider: Collider,
    name: Name,
    movement: Movement,
    punch_timer: PunchTimer,
    direction: Direction,
    health: Health,
    inventory: Inventory,
}

fn setup_player(mut commands: Commands, textures: Res<TextureAssets>) {
    let mut animation_manager = AnimationManager::new(
        vec![
            // Idle
            Animation::new(AnimationBounds::new(0, 0), Duration::from_millis(500)),
            // Walking
            Animation::new(AnimationBounds::new(1, 8), Duration::from_millis(120)),
            // Punching
            Animation::new(AnimationBounds::new(9, 11), Duration::from_millis(120)),
        ],
        0,
    );

    animation_manager.add_state("walk".to_string(), false);
    animation_manager.add_state("punch".to_string(), false);

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
        AnimationTransitionCondition::new(Box::new(|state| state["punch"]))
            .with_mode(AnimationTransitionMode::Immediate),
    );
    animation_manager.add_graph_edge(
        1,
        2,
        AnimationTransitionCondition::new(Box::new(|state| state["punch"]))
            .with_mode(AnimationTransitionMode::Immediate),
    );
    animation_manager.add_graph_edge(2, 0, AnimationTransitionCondition::new(Box::new(|_| true)));

    commands.spawn(PlayerBundle {
        player: Player::default(),
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: textures.player_down.clone(),
            transform: Transform::from_xyz(0., 0., 5.),
            ..Default::default()
        },
        animation_manager,
        rigidbody: RigidBody::KinematicPositionBased,
        collider: Collider::cuboid(21., 53.),
        name: Name::new("Player"),
        movement: Movement {
            speed: 500.0, // TODO: Change it to 200.0 for release
            running_speed: 250.0,
        },
        punch_timer: PunchTimer(Timer::from_seconds(2.5, TimerMode::Once)),
        direction: Direction::Down,
        health: Health::default(),
        inventory: Inventory::new(3),
    });
}

fn player_movement(
    mut player_query: Query<
        (
            Entity,
            &mut Transform,
            &mut Direction,
            &mut AnimationManager,
            &Collider,
            &Movement,
            Option<&MovementBoost>,
            Option<&Dizziness>,
        ),
        With<Player>,
    >,
    rapier_context: Res<RapierContext>,
    actions: Res<Actions>,
    time: Res<Time>,
) {
    for (
        entity,
        mut transform,
        mut direction,
        mut animation_manager,
        collider,
        movement,
        movement_boost,
        dizziness,
    ) in player_query.iter_mut()
    {
        let speed = movement.speed * time.delta_seconds();

        let movement_vector = actions.player_movement.normalize_or_zero()
            * speed
            * if let Some(movement_boost) = movement_boost {
                movement_boost.multiplier
            } else {
                1.0
            }
            * if dizziness.is_some() { -1.0 } else { 1.0 };

        if movement_vector != Vec2::ZERO {
            let angle = movement_vector.angle_between(Vec2::new(0., 1.));

            *direction = Direction::from(Euler::from_radians(angle));
        }

        let horizontal_vector = Vec2::new(movement_vector.x, 0.);
        let vertical_vector = Vec2::new(0., movement_vector.y);

        let horizontal_target = {
            if let Some((_entity, hit)) = rapier_context.cast_shape(
                transform.translation.truncate(),
                0.,
                horizontal_vector,
                collider,
                1.,
                QueryFilter::default()
                    .exclude_sensors()
                    .exclude_collider(entity),
            ) {
                transform.translation.truncate() + horizontal_vector * (hit.toi - 1.).max(0.)
            } else {
                transform.translation.truncate() + horizontal_vector
            }
        };

        let vertical_target = {
            if let Some((_entity, hit)) = rapier_context.cast_shape(
                transform.translation.truncate(),
                0.,
                vertical_vector,
                collider,
                1.,
                QueryFilter::default()
                    .exclude_sensors()
                    .exclude_collider(entity),
            ) {
                transform.translation.truncate() + vertical_vector * (hit.toi - 1.).max(0.)
            } else {
                transform.translation.truncate() + vertical_vector
            }
        };

        let target = Vec3::new(
            horizontal_target.x,
            vertical_target.y,
            transform.translation.z,
        );

        animation_manager
            .set_state("walk".to_string(), target != transform.translation)
            .unwrap();

        transform.translation = target;
    }
}

fn update_sprite(
    mut player_query: Query<(&mut Handle<TextureAtlas>, &Direction), With<Player>>,
    textures: Res<TextureAssets>,
) {
    for (mut sprite, direction) in player_query.iter_mut() {
        *sprite = match direction {
            Direction::Up => textures.player_up.clone(),
            Direction::Down => textures.player_down.clone(),
            Direction::Left => textures.player_left.clone(),
            Direction::Right => textures.player_right.clone(),
        };
    }
}

fn spawn_player_body(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform, &Health), With<Player>>,
    textures: Res<TextureAssets>,
) {
    for (player_entity, player_transform, player_health) in player_query.iter() {
        if player_health.get_health() <= 0. {
            commands.entity(player_entity).despawn_recursive();
            commands
                .spawn(SpriteBundle {
                    transform: player_transform.clone(),
                    texture: textures.player_body.clone(),
                    ..Default::default()
                })
                .insert(PlayerBody);
        }
    }
}

fn punch_enemies(
    mut player_query: Query<
        (
            &mut PunchTimer,
            &mut AnimationManager,
            &Transform,
            &Direction,
        ),
        With<Player>,
    >,
    mut enemy_query: Query<(&mut EnemyState, &Transform)>,
    mut burst_actions: EventReader<BurstActions>,
    time: Res<Time>,
) {
    let (mut punch_timer, mut animation_manager, player_transform, player_direction) =
        player_query.single_mut();

    animation_manager
        .set_state("punch".to_string(), false)
        .unwrap();

    if !punch_timer.tick(time.delta()).finished() {
        return;
    }

    for action in burst_actions.iter() {
        if *action != BurstActions::Punch {
            continue;
        }

        for (mut enemy_state, _) in enemy_query.iter_mut().filter(|(_, enemy_transform)| {
            let vector =
                enemy_transform.translation.truncate() - player_transform.translation.truncate();

            let angle = vector.angle_between(Vec2::new(0., 1.));

            vector.length() < 50.0
                && *player_direction == Direction::from(Euler::from_radians(angle))
        }) {
            *enemy_state = EnemyState::Stun {
                timer: Timer::from_seconds(1.5, TimerMode::Once),
            };
        }

        animation_manager
            .set_state("punch".to_string(), true)
            .unwrap();

        punch_timer.reset();
    }
}

fn damage_yourself(
    mut player_query: Query<&mut Health, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
) {
    let mut player_health = player_query.single_mut();

    #[allow(clippy::collapsible_if)]
    if cfg!(debug_assertions) && keyboard.just_pressed(KeyCode::Q) {
        if *player_health.take_damage(10.0) {
            state.set(GameState::GameOver);
        }
    }
}

fn pick_up_pills(
    mut commands: Commands,
    pill_query: Query<(Entity, &Pill)>,
    mut player_query: Query<(Entity, &mut Inventory), With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    let (player_entity, mut inventory) = player_query.single_mut();

    for (pill_entity, pill) in pill_query.iter() {
        if rapier_context.intersection_pair(pill_entity, player_entity) == Some(true)
            && inventory.add_pill(*pill)
        {
            commands.entity(pill_entity).despawn();
        }
    }
}

pub fn consume_pills(
    mut player_query: Query<&mut Inventory, With<Player>>,
    mut burst_actions: EventReader<BurstActions>,
) -> Vec<Pill> {
    let mut inventory = player_query.single_mut();

    let mut pills = Vec::new();

    for action in burst_actions.iter() {
        match action {
            BurstActions::ConsumePill { index } => {
                let pill = if let Some(pill) = inventory.consume_pill(*index) {
                    pill
                } else {
                    continue;
                };

                pills.push(pill);
            }
            _ => {}
        }
    }

    pills
}
