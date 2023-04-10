use bevy::prelude::*;
use bevy_spritesheet_animation::animation_manager::{transition_animations, AnimationManager};

use crate::{
    player::{
        effect::{Invincibility, Vulnerability},
        Player,
    },
    unit::Health,
    GameState,
};

use super::{sight::see_player, EnemyState};

pub struct EnemyAttackPlugin;

impl Plugin for EnemyAttackPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EnemyAttackTimer>().add_system(
            attack_player
                .after(see_player)
                .after(transition_animations)
                .in_set(OnUpdate(GameState::Playing)),
        );
    }
}

#[derive(Component, Debug, Clone, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct EnemyAttackTimer(pub Timer);

impl Default for EnemyAttackTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1., TimerMode::Repeating))
    }
}

fn attack_player(
    mut enemy_query: Query<(
        &mut EnemyState,
        &mut EnemyAttackTimer,
        &mut AnimationManager,
        &Transform,
    )>,
    mut player_query: Query<
        (
            &Transform,
            &mut Health,
            Option<&Invincibility>,
            Option<&Vulnerability>,
        ),
        With<Player>,
    >,
    time: Res<Time>,
    mut state: ResMut<NextState<GameState>>,
) {
    let (player_transform, mut player_health, player_invincibility, player_vulnerability) =
        player_query.single_mut();

    for (mut enemy_state, mut enemy_timer, mut animation_manager, enemy_transform) in
        enemy_query.iter_mut()
    {
        animation_manager
            .set_state("shoot".to_string(), false)
            .unwrap();

        if matches!(*enemy_state, EnemyState::Stun { .. }) {
            continue;
        }

        if matches!(*enemy_state, EnemyState::Alert { .. })
            && player_transform
                .translation
                .truncate()
                .distance(enemy_transform.translation.truncate())
                < 25.
        {
            enemy_timer.tick(time.delta());

            if enemy_timer.just_finished() {
                if *player_health.take_damage(
                    (rand::random::<f32>() * 5.0
                        + 20.0
                            * if let Some(vulnerability) = player_vulnerability {
                                vulnerability.amount
                            } else {
                                1.0
                            })
                        * if player_invincibility.is_some() {
                            0.0
                        } else {
                            1.0
                        },
                ) {
                    state.set(GameState::GameOver);

                    *enemy_state = EnemyState::Idle;
                }

                animation_manager
                    .set_state("shoot".to_string(), true)
                    .unwrap();
            }
        }
    }
}
