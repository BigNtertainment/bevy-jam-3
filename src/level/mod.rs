use crate::enemy::{ENEMY_COLLIDER_HEIGHT, ENEMY_COLLIDER_WIDTH};
use crate::GameState;
use crate::pill::PillBundle;
use crate::{enemy::EnemyBundle, loading::LevelAssets};
use ::navmesh::NavMesh;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::{TilemapGridSize, TilemapSize};
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_rapier2d::prelude::{Collider, PhysicsSet};

use self::navmesh::{draw_nav_mesh, NavMeshBuilder};

mod navmesh;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .configure_set(LdtkSystemSet::ProcessApi.before(PhysicsSet::SyncBackend))
            .insert_resource(LevelSelection::Uid(0))
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                level_background: LevelBackground::Nonexistent,
                ..Default::default()
            })
            .register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_int_cell::<NavMeshCellBundle>(2)
            .register_ldtk_entity::<EnemyBundle>("Enemy")
            .register_ldtk_entity::<PillBundle>("Pill")
            .register_ldtk_entity::<PlayerSpawnBundle>("PlayerSpawn")
            .add_system(ldtk_setup.in_schedule(OnEnter(GameState::Playing)))
            .add_system(generate_nav_mesh.in_set(OnUpdate(GameState::Playing)));

        #[cfg(debug_assertions)]
        {
            app.add_system(draw_nav_mesh.in_set(OnUpdate(GameState::Playing)));
        }
    }
}

#[derive(Component, Reflect, Default, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub struct Wall;

pub fn grid_collider(grid_cell: IntGridCell) -> Collider {
    match grid_cell.value {
        1 => Collider::cuboid(32.0, 32.0),
        _ => Collider::cuboid(1.0, 1.0),
    }
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
    #[with(grid_collider)]
    collider: Collider,
}

#[derive(Component, Reflect, Default, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub struct NavMeshCell;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct NavMeshCellBundle {
    nav_mesh_cell: NavMeshCell,
    transform: Transform,
    global_transform: GlobalTransform,
}

#[derive(Component, Reflect, Default, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub struct PlayerSpawn;

#[derive(Clone, Debug, Default, Bundle, LdtkEntity)]
pub struct PlayerSpawnBundle {
    player_spawn: PlayerSpawn,
    transform: Transform,
    global_transform: GlobalTransform,
}

#[derive(Component, Default, Deref, DerefMut, Clone, Debug)]
pub struct WorldNavMesh(pub NavMesh);

fn generate_nav_mesh(
    mut commands: Commands,
    nav_mesh_cell_query: Query<(&TilePos, &GlobalTransform), With<NavMeshCell>>,
    level_query: Query<(&TilemapGridSize, &TilemapSize)>,
    mut level_events: EventReader<LevelEvent>,
) {
    for event in level_events.iter() {
        match event {
            LevelEvent::Transformed(_iid) => {
                let (grid_size, tilemap_size) = level_query.single();

                let mut nav_mesh_builder = NavMeshBuilder::new();

                let width = tilemap_size.x;
                let height = tilemap_size.y;
                let mut map = HashMap::new();

                for (nav_mesh_cell, cell_transform) in nav_mesh_cell_query.iter() {
                    map.insert(
                        (nav_mesh_cell.x, nav_mesh_cell.y),
                        cell_transform.translation().truncate(),
                    );
                }

                for x in 1..=width {
                    for y in 1..=height {
                        if !map.contains_key(&(x, y)) {
                            continue;
                        }

                        let top = -(grid_size.y
                            - if !map.contains_key(&(x, y - 1)) {
                                ENEMY_COLLIDER_HEIGHT
                            } else {
                                0.0
                            })
                            / 2.;

                        let bottom = (grid_size.y
                            - if !map.contains_key(&(x, y + 1)) {
                                ENEMY_COLLIDER_HEIGHT
                            } else {
                                0.0
                            })
                            / 2.;

                        let left = -(grid_size.x
                            - if !map.contains_key(&(x - 1, y)) {
                                ENEMY_COLLIDER_WIDTH
                            } else {
                                0.0
                            })
                            / 2.;

                        let right = (grid_size.x
                            - if !map.contains_key(&(x + 1, y)) {
                                ENEMY_COLLIDER_WIDTH
                            } else {
                                0.0
                            })
                            / 2.;

                        let tile_pos = map[&(x, y)];

                        nav_mesh_builder.insert_rect(
                            tile_pos + Vec2::new(left, top),
                            tile_pos + Vec2::new(right, top),
                            tile_pos + Vec2::new(right, bottom),
                            tile_pos + Vec2::new(left, bottom),
                        );
                    }
                }

                let nav_mesh = nav_mesh_builder.bake();

                commands.spawn((TransformBundle::default(), WorldNavMesh(nav_mesh)));
            }
            _ => {}
        }
    }
}

fn ldtk_setup(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: level_assets.ldtk_handle.clone(),
        ..Default::default()
    });
}
