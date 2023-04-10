use crate::{loading::LevelAssets, enemy::EnemyBundle};
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
            .register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_entity::<EnemyBundle>("Enemy")
            .add_system(ldtk_setup.in_schedule(OnEnter(GameState::Playing)));
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

fn ldtk_setup(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: level_assets.ldtk_handle.clone(),
        ..Default::default()
    });
}
