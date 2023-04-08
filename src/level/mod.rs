use crate::loading::LevelAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::{PhysicsSet, Collider};

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
            .register_ldtk_int_cell::<WallMarkerBundle>(1)
            .add_system(ldtk_setup.in_schedule(OnEnter(GameState::Playing)))
            .add_system(spawn_walls.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component, Reflect, Default, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallMarkerBundle {
    wall: Wall,
}

#[derive(Bundle)]
pub struct WallBundle {
    collider: Collider,
}

fn ldtk_setup(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: level_assets.ldtk_handle.clone(),
        ..Default::default()
    });
}

fn spawn_walls(
    mut commands: Commands,
    wall_query: Query<Entity, Added<Wall>>,
) {
    for wall in wall_query.iter() {
        commands.entity(wall).insert(WallBundle {
            collider: Collider::cuboid(32., 32.),
        });
    }
}
