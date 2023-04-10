use bevy::prelude::*;

use crate::{GameState, player::Player};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_setup.in_schedule(OnEnter(GameState::Loading)))
            .add_system(follow_player.in_set(OnUpdate(GameState::Playing)));
    }
}

fn camera_setup(mut commands: Commands) {
    // This assumes we only have a single window
    commands.spawn(Camera2dBundle::default());
}

fn follow_player(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();

    camera_transform.translation = player_transform
        .translation
        .truncate()
        .extend(camera_transform.translation.z);
}
