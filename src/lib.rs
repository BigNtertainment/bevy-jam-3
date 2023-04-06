mod actions;
mod cleanup;
mod enemy;
mod game_over;
mod loading;
mod menu;
mod pill;
mod player;
mod unit;
mod world;

use actions::ActionsPlugin;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pathmesh::PathMeshPlugin;
use bevy_rapier2d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use enemy::EnemyPlugin;
use game_over::GameOverPlugin;
use loading::LoadingPlugin;
use menu::MenuPlugin;
use pill::PillPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    // Here the menu is drawn after player died but
    GameOver,
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum WorldState {
    // World is not rendered
    #[default]
    No,
    // World is rendered
    Yes,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_state::<WorldState>()
            .add_plugin(LoadingPlugin)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(PathMeshPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(GameOverPlugin)
            .add_plugin(PillPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(WorldPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(RapierDebugRenderPlugin::default())
                .add_plugin(WorldInspectorPlugin::default());
        }
    }
}
