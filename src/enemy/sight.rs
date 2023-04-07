use bevy::prelude::*;
use bevy_rapier2d::prelude::{QueryFilter, RapierContext};

use crate::{
    player::Player,
    unit::{Direction, Euler},
    GameState,
};

use super::EnemyState;

pub struct EnemySightPlugin;

impl Plugin for EnemySightPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(see_player.in_set(OnUpdate(GameState::Playing)));
    }
}

fn see_player(
    mut enemy_query: Query<(&mut EnemyState, &Transform, &Direction)>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    let (player_entity, player_transform) = player_query.single();

    for (mut enemy_state, enemy_transform, enemy_direction) in enemy_query.iter_mut() {
        let to_player_vector =
            (player_transform.translation - enemy_transform.translation).truncate();

        if *enemy_state.as_ref() == EnemyState::Idle {
            let angle = Euler::from_radians(to_player_vector.angle_between(Vec2::new(0., 1.)));

            if Direction::from(angle) != *enemy_direction {
                continue;
            }
        }

        if let Some((entity, _)) = rapier_context.cast_ray(
            enemy_transform.translation.truncate(),
            to_player_vector.normalize(),
            35000.0,
            true,
            QueryFilter::new().exclude_sensors(),
        ) {
            if entity == player_entity {
                *enemy_state = EnemyState::Alert {
                    target: player_transform.translation.truncate(),
                };
            }
        }
    }
}
