use bevy::prelude::*;
use bevy_rapier2d::prelude::{QueryFilter, RapierContext};

use crate::{
    player::Player,
    unit::{Direction, Euler},
    GameState,
};

use super::{super::player::effect::Invisibility, movement::enemy_movement, EnemyState};

pub struct EnemySightPlugin;

impl Plugin for EnemySightPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            see_player
                .after(enemy_movement)
                .in_set(OnUpdate(GameState::Playing)),
        );
    }
}

pub fn see_player(
    mut enemy_query: Query<(&mut EnemyState, &Transform, &Direction)>,
    player_query: Query<(Entity, &Transform, Option<&Invisibility>), With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    let (player_entity, player_transform, player_invisibility) = player_query.single();

    for (mut enemy_state, enemy_transform, enemy_direction) in enemy_query.iter_mut() {
        if matches!(*enemy_state, EnemyState::Stun { .. }) {
            continue;
        }

        let to_player_vector =
            (player_transform.translation - enemy_transform.translation).truncate();

        let distance = player_transform
            .translation
            .truncate()
            .distance(enemy_transform.translation.truncate());

        let angle = Euler::from_radians(to_player_vector.angle_between(Vec2::new(0., 1.)));

        let enemy_sight = 35000.0;

        let see_player = distance < 15.
            || (if matches!(*enemy_state.as_ref(), EnemyState::Idle) {
                Direction::from(angle) == *enemy_direction
            } else {
                true
            } && if let Some((entity, _)) = rapier_context.cast_ray(
                enemy_transform.translation.truncate(),
                to_player_vector.normalize(),
                enemy_sight,
                true,
                QueryFilter::new().exclude_sensors(),
            ) {
                entity == player_entity
            } else {
                false
            } && player_invisibility.is_none());

        if see_player {
            *enemy_state = EnemyState::Alert {
                target: player_transform.translation.truncate(),
            };
        } else {
            *enemy_state = EnemyState::Idle;
        }
    }
}
