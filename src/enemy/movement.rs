use bevy::prelude::*;
use bevy_pathmesh::PathMesh;

use crate::{
    unit::{Direction, Euler, Movement},
    world::{NavMesh, World},
    GameState,
};

use super::EnemyState;

pub struct EnemyMovementPlugin;

impl Plugin for EnemyMovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EnemyMovementTarget>().add_systems(
            (enemy_movement, enemy_guard_area_timer).in_set(OnUpdate(GameState::Playing)),
        );
    }
}

#[derive(Reflect, Component, Clone, Default, Debug, PartialEq)]
#[reflect(Component)]
pub struct EnemyMovementTarget {
    pub path: Vec<Vec2>,
}

#[derive(Component, Clone, Debug)]
pub enum EnemyMovementType {
    Static {
        target: Vec2,
    },
    AlongPath {
        path: Vec<Vec2>,
        current: usize,
    },
    GuardArea {
        area: Rect,
        current: Vec2,
        wait_timer: Timer,
    },
}

impl EnemyMovementType {
    fn move_to_next(&mut self) {
        match self {
            EnemyMovementType::Static { target: _ } => (),
            EnemyMovementType::AlongPath { path, current } => {
                *current = (*current + 1) % path.len();
            }
            EnemyMovementType::GuardArea {
                area,
                current,
                wait_timer: _,
            } => {
                let random_point = area.center()
                    + Vec2::new(
                        (rand::random::<f32>() - 0.5) * area.width(),
                        (rand::random::<f32>() - 0.5) * area.height(),
                    );

                *current = random_point;
            }
        }
    }

    fn target(&self) -> Option<Vec2> {
        match self {
            EnemyMovementType::Static { target } => Some(*target),
            EnemyMovementType::AlongPath { path, current } => Some(path[*current]),
            EnemyMovementType::GuardArea {
                area: _,
                current,
                wait_timer,
            } => {
                if wait_timer.just_finished() {
                    Some(*current)
                } else {
                    None
                }
            }
        }
    }
}

fn enemy_movement(
    mut enemy_query: Query<(
        &mut EnemyState,
        &mut EnemyMovementTarget,
        &mut EnemyMovementType,
        &Movement,
        &mut Direction,
        &mut Transform,
    )>,
    nav_mesh_query: Query<&NavMesh, With<World>>,
    mesh_assets: Res<Assets<PathMesh>>,
    time: Res<Time>,
) {
    let nav_mesh = nav_mesh_query.single();

    for (
        mut enemy_state,
        mut enemy_movement_target,
        mut enemy_movement_type,
        enemy_movement,
        mut enemy_direction,
        mut enemy_transform,
    ) in enemy_query.iter_mut()
    {
        let new_target = match *enemy_state {
            EnemyState::Idle => {
                if enemy_movement_target.path.is_empty() {
                    enemy_movement_type.move_to_next();
                    let target = enemy_movement_type
                        .target()
                        .unwrap_or(enemy_transform.translation.truncate());

                    Some(target)
                } else {
                    None
                }
            }
            EnemyState::Alert { target } => {
                if matches!(*enemy_state, EnemyState::Alert { .. })
                    && enemy_movement_target.path.is_empty()
                {
                    *enemy_state = EnemyState::Idle;

                    continue;
                }

                Some(target)
            }
            EnemyState::Attacking => continue,
        };

        if let Some(target) = new_target {
            if let Some(path) = mesh_assets
                .get(&nav_mesh.0)
                .unwrap()
                .path(enemy_transform.translation.truncate(), target)
            {
                enemy_movement_target.path = path.path;
            } else {
                eprintln!(
                    "No path found between points {} and {}",
                    enemy_transform.translation.truncate(),
                    target
                );
            }
        }

        if let Some(target) = enemy_movement_target.path.get(0) {
            let movement_vector = *target - enemy_transform.translation.truncate();

            let calc_movement_vector =
                movement_vector.normalize_or_zero() * enemy_movement.speed * time.delta_seconds();

            let movement_vector = if calc_movement_vector.length() > movement_vector.length() {
                movement_vector
            } else {
                calc_movement_vector
            };

            enemy_transform.translation += movement_vector.extend(0.0);

            if movement_vector != Vec2::ZERO {
                let movement_angle =
                    Euler::from_radians(movement_vector.angle_between(Vec2::new(0.0, 1.0)));
                enemy_direction.set_if_neq(Direction::from(movement_angle));
            }

            if (*target - enemy_transform.translation.truncate()).length() < 0.1 {
                enemy_movement_target.path.remove(0);
            }
        }
    }
}

fn enemy_guard_area_timer(
    mut enemy_query: Query<(&mut EnemyMovementType, &EnemyMovementTarget, &Transform)>,
    time: Res<Time>,
) {
    for (mut movement_type, target, transform) in enemy_query.iter_mut() {
        if let EnemyMovementType::GuardArea {
            area: _,
            current: _,
            wait_timer,
        } = movement_type.as_mut()
        {
            let target = target.path.get(0).map(|target| *target).unwrap_or(transform.translation.truncate());

            if transform.translation.truncate() == target {
                wait_timer.tick(time.delta());
            } else {
                wait_timer.reset();
            }
        }
    }
}
