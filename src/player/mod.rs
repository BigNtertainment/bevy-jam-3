use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, QueryFilter, RapierContext, Sensor};

use crate::{
    actions::Actions,
    cleanup::cleanup,
    loading::TextureAssets,
    unit::{Health, Movement},
    GameState, WorldState,
};

use self::ui::{setup_ui, update_ui, PlayerUI};

mod ui;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_systems((setup_player, setup_ui).in_schedule(OnEnter(WorldState::Yes)))
            .add_systems(
                (player_movement, update_ui, damage_yourself).in_set(OnUpdate(GameState::Playing)),
            )
            .add_system(
                cleanup::<Player>.in_schedule(OnExit(WorldState::No)),
            )
            .add_system(cleanup::<PlayerUI>.in_schedule(OnExit(GameState::Playing)));
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
    collider: Collider,
    sensor: Sensor,
    name: Name,
    movement: Movement,
    health: Health,
}

fn setup_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn(PlayerBundle {
        player: Player::default(),
        sprite_bundle: SpriteBundle {
            texture: textures.player.clone(),
            ..Default::default()
        },
        collider: Collider::cuboid(27., 63.),
        sensor: Sensor,
        name: Name::new("Player"),
        movement: Movement { speed: 400.0 },
        health: Health::default(),
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

fn damage_yourself(
    mut player_query: Query<&mut Health, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
) {
    let mut player_health = player_query.single_mut();

    #[allow(clippy::collapsible_if)]
    if cfg!(debug_assertions) && keyboard.just_pressed(KeyCode::Space) {
        if *player_health.take_damage(10.0) {
            state.set(GameState::GameOver)
        }
    }
}
