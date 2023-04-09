use bevy::prelude::*;

use crate::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_setup.in_schedule(OnEnter(GameState::Loading)));
    }
}

fn camera_setup(mut commands: Commands) {
    // This assumes we only have a single window
    commands.spawn(Camera2dBundle::default());
}
