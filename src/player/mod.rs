use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, QueryFilter, RapierContext, Sensor, RigidBody};

use crate::{
    actions::{Actions, BurstActions},
    cleanup::cleanup,
    loading::TextureAssets,
    pill::Pill,
    unit::{Health, Movement},
    GameState,
};

use self::{ui::{setup_ui, update_health_ui, PlayerUI, HealthUI, InventorySlotUI, update_inventory_ui}, inventory::Inventory};

mod ui;
mod inventory;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .register_type::<PlayerUI>()
            .register_type::<HealthUI>()
            .register_type::<InventorySlotUI>()
            .add_systems((setup_player, setup_ui).in_schedule(OnEnter(GameState::Playing)))
            .add_systems((player_movement, pick_up_pills, consume_pills, update_health_ui, update_inventory_ui).in_set(OnUpdate(GameState::Playing)))
            .add_systems(
                (cleanup::<Player>, cleanup::<PlayerUI>).in_schedule(OnExit(GameState::Playing)),
            );
    }
}

#[derive(Reflect, Component, Copy, Clone, Default, Debug, PartialEq, Eq)]
#[reflect(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    #[bundle]
    sprite_bundle: SpriteBundle,
    // TODO: make an issue in rapier so they register their types
    rigidbody: RigidBody,
    collider: Collider,
    sensor: Sensor,
    name: Name,
    movement: Movement,
    health: Health,
    inventory: Inventory,
}

fn setup_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn(PlayerBundle {
        player: Player::default(),
        sprite_bundle: SpriteBundle {
            texture: textures.player.clone(),
            transform: Transform::from_xyz(0., 0., 5.),
            ..Default::default()
        },
        rigidbody: RigidBody::KinematicPositionBased,
        collider: Collider::cuboid(27., 63.),
        sensor: Sensor,
        name: Name::new("Player"),
        movement: Movement { speed: 400.0 },
        health: Health::default(),
        inventory: Inventory::new(3),
    });
}

fn player_movement(
    mut player_query: Query<(&mut Transform, &Collider, &Movement), With<Player>>,
    rapier_context: Res<RapierContext>,
    actions: Res<Actions>,
    time: Res<Time>,
) {
    for (mut transform, collider, movement) in player_query.iter_mut() {
        let speed = movement.speed * time.delta_seconds();

        let movement_vector = actions.player_movement.normalize_or_zero() * speed;

        let horizontal_vector = Vec2::new(movement_vector.x, 0.);
        let vertical_vector = Vec2::new(0., movement_vector.y);

        let horizontal_target = {
            if let Some((_entity, hit)) = rapier_context.cast_shape(
                transform.translation.truncate(),
                0.,
                horizontal_vector,
                collider,
                1.,
                QueryFilter::default().exclude_sensors(),
            ) {
                transform.translation.truncate() + horizontal_vector * (hit.toi - 0.5).max(0.)
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
                QueryFilter::default().exclude_sensors(),
            ) {
                transform.translation.truncate() + vertical_vector * (hit.toi - 0.5).max(0.)
            } else {
                transform.translation.truncate() + vertical_vector
            }
        };

        let target = Vec3::new(horizontal_target.x, vertical_target.y, 0.);

        transform.translation = target;
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
) {
    let mut inventory = player_query.single_mut();

    for action in burst_actions.iter() {
        match action {
            BurstActions::ConsumePill { index } => {
                let pill = if let Some(pill) = inventory.consume_pill(*index) {
                    pill
                } else {
                    continue
                };

                println!("Consumed pill: {:?}, {:?}", pill.main_effect, pill.side_effect);
            },
            _ => {}
        }
    }
}