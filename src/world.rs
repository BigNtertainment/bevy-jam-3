use bevy::prelude::*;
use bevy_pathmesh::PathMesh;

use crate::{cleanup::cleanup, WorldState};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<World>()
            .register_type::<NavMesh>()
            .add_system(world_setup.in_schedule(OnEnter(WorldState::Yes)))
            .add_system(cleanup::<World>.in_schedule(OnExit(WorldState::Yes)));
    }
}

#[derive(Component, Reflect, Default, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub struct World;

#[derive(Component, Reflect, Default, Deref, DerefMut, Clone, Debug, PartialEq, Eq)]
#[reflect(Component)]
pub struct NavMesh(pub Handle<PathMesh>);

// TODO: Move this over to the level module
fn world_setup(
    mut commands: Commands,
    mut pathmeshes: ResMut<Assets<PathMesh>>,
) {
    // TODO: Precompute the navmesh and load it from a file (see https://github.com/vleue/bevy_pathmesh/blob/main/assets/arena-merged.polyanya.mesh)
    let navmesh = pathmeshes.add(PathMesh::from_polyanya_mesh(polyanya::Mesh::new(
        vec![
            polyanya::Vertex::new(Vec2::new(-500., -500.), vec![0, -1]),
            polyanya::Vertex::new(Vec2::new(500., -500.), vec![0, -1]),
            polyanya::Vertex::new(Vec2::new(500., 500.), vec![0, -1]),
            polyanya::Vertex::new(Vec2::new(-500., 500.), vec![0, -1]),
        ],
        vec![polyanya::Polygon::new(vec![0, 1, 2, 3], false)],
    )));

    commands
        .spawn((
            TransformBundle::default(),
            VisibilityBundle::default(),
            Name::new("World"),
            NavMesh(navmesh),
            World,
        ));
}
