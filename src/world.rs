use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::{cleanup::cleanup, GameState, loading::TextureAssets};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(world_setup.in_schedule(OnEnter(GameState::Playing)))
            .add_system(cleanup::<World>.in_schedule(OnExit(GameState::Playing)));
    }
}

#[derive(Component, Reflect, Default, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub struct World;

fn world_setup(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
			texture: textures.wall.clone(),
            transform: Transform {
                translation: Vec3::new(200., 200., 0.),
                ..Default::default()
            },
            ..default()
        })
		.insert(Name::new("Wall"))
        .insert(World)
        .insert(Collider::cuboid(32., 32.));
}
