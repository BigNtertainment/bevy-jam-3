use bevy::prelude::*;

use crate::{player::Player, unit::Health, GameState};

use super::{sight::see_player, EnemyState};

pub struct EnemyAttackPlugin;

impl Plugin for EnemyAttackPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EnemyAttackTimer>().add_system(
            attack_player
                .after(see_player)
                .in_set(OnUpdate(GameState::Playing)),
        );
    }
}

#[derive(Component, Debug, Clone, Default, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct EnemyAttackTimer(pub Timer);

fn attack_player(
    mut enemy_query: Query<(&EnemyState, &mut EnemyAttackTimer, &Transform)>,
    mut player_query: Query<(&Transform, &mut Health), With<Player>>,
    time: Res<Time>,
    mut state: ResMut<NextState<GameState>>,
) {
    let (player_transform, mut player_health) = player_query.single_mut();

    for (enemy_state, mut enemy_timer, enemy_transform) in enemy_query.iter_mut() {
        if matches!(enemy_state, EnemyState::Alert { .. })
            && player_transform
                .translation
                .truncate()
                .distance(enemy_transform.translation.truncate())
                < 15.
        {
            enemy_timer.tick(time.delta());

            if enemy_timer.just_finished()
                && *player_health.take_damage(rand::random::<f32>() * 5.0 + 20.0)
            {
                state.set(GameState::GameOver);
            }
        }
    }
}
