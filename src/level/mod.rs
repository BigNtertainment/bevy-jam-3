use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::loading::LevelAssets;
use crate::GameState;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            // Required to prevent race conditions between bevy_ecs_ldtk's and bevy_rapier's
            // systems
            .configure_set(LdtkSystemSet::ProcessApi.before(PhysicsSet::SyncBackend))
            .insert_resource(LevelSelection::Index(0))
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                set_clear_color: SetClearColor::FromLevelBackground,
                ..Default::default()
            })
            .add_system(ldtk_setup.in_schedule(OnEnter(GameState::Menu)))
            .add_system(
                spawn_wall_collision
                    .after(ldtk_setup)
                    .in_set(OnUpdate(GameState::Playing)),
            )
            .register_ldtk_int_cell::<WallBundle>(1);
    }
}

fn ldtk_setup(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: level_assets.ldtk_handle.clone(),
        ..Default::default()
    });
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

/// Spawns rectangle colliders that cover more than a single wall tile (better performance).
fn spawn_wall_collision(
    mut commands: Commands,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    wall_query: Query<&GridCoords, Added<Wall>>,
    levels: Res<Assets<LdtkLevel>>,
) {
    // Some setup
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // TODO: Maybe this can be done in a single step
    let mut level_walls = HashSet::new();
    wall_query.for_each(|&grid_coords| {
        level_walls.insert(grid_coords);
    });

    if let Ok((level_entity, level_handle)) = level_query.get_single() {
        let level = levels.get(level_handle).expect("Faile to get level");

        let LayerInstance {
            c_wid: width,
            c_hei: height,
            grid_size,
            ..
        } = level
            .level
            .layer_instances
            .clone()
            .expect("Level asset should have layers")[0];

        // Group walls together
        let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

        for y in 0..height {
            let mut row_plates: Vec<Plate> = Vec::new();
            let mut plate_start = None;

            for x in 0..width + 1 {
                match (plate_start, level_walls.contains(&GridCoords { y, x })) {
                    (Some(s), false) => {
                        row_plates.push(Plate {
                            left: s,
                            right: x - 1,
                        });
                        plate_start = None;
                    }
                    (None, true) => plate_start = Some(x),
                    _ => {}
                }
            }
        }

        let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
        let mut prev_row: Vec<Plate> = Vec::new();
        let mut wall_rects: Vec<Rect> = Vec::new();

        plate_stack.push(Vec::new());

        // This code does something ..
        for (y, current_row) in plate_stack.into_iter().enumerate() {
            for prev_plate in &prev_row {
                if !current_row.contains(prev_plate) {
                    if let Some(rect) = rect_builder.remove(prev_plate) {
                        wall_rects.push(rect);
                    }
                }
            }

            for plate in &current_row {
                rect_builder
                    .entry(plate.clone())
                    .and_modify(|e| e.top += 1)
                    .or_insert(Rect {
                        bottom: y as i32,
                        top: y as i32,
                        left: plate.left,
                        right: plate.right,
                    });
            }

            prev_row = current_row;
        }

        println!("{}", wall_rects.len());

        // And this one too ..
        commands.entity(level_entity).with_children(|level| {
            for wall_rect in wall_rects {
                level
                    .spawn_empty()
                    .insert(Name::new("Penis"))
                    .insert(Collider::cuboid(
                        (wall_rect.right as f32 - wall_rect.left as f32 + 1.) * grid_size as f32
                            / 2.,
                        (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.) * grid_size as f32
                            / 2.,
                    ))
                    .insert(RigidBody::Fixed)
                    .insert(Friction::new(1.0))
                    .insert(Transform::from_xyz(
                        (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32 / 2.,
                        (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32 / 2.,
                        0.,
                    ))
                    .insert(GlobalTransform::default());
            }
        });
    } else {
        println!("sperm");
    }
}
