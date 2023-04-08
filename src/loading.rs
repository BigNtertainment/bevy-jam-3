use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
    #[asset(path = "fonts/SpaceGrotesk-Medium.ttf")]
    pub space_grotesk: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    // TODO: Change this texture
    #[asset(path = "textures/player.png")]
    pub player: Handle<Image>,
    // TODO: Change this texture
    #[asset(path = "textures/wall.png")]
    pub wall: Handle<Image>,
    #[asset(texture_atlas(tile_size_x=200., tile_size_y=260., columns=5, rows=6))]
    #[asset(path = "textures/enemy/enemy-up.png")]
    pub enemy_up: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x=200., tile_size_y=300., columns=5, rows=4))]
    #[asset(path = "textures/enemy/enemy-down.png")]
    pub enemy_down: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x=640., tile_size_y=480., columns=5, rows=6))]
    #[asset(path = "textures/enemy/enemy-left.png")]
    pub enemy_left: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x=640., tile_size_y=480., columns=5, rows=6))]
    #[asset(path = "textures/enemy/enemy-right.png")]
    pub enemy_right: Handle<TextureAtlas>,
    #[asset(path = "textures/pills/health_pill.png")]
    pub health_pill: Handle<Image>,
    #[asset(path = "textures/pills/speed_pill.png")]
    pub speed_pill: Handle<Image>,
    #[asset(path = "textures/pills/toxic_fart_pill.png")]
    pub toxic_fart_pill: Handle<Image>,
    #[asset(path = "textures/pills/invisibility_pill.png")]
    pub invisibility_pill: Handle<Image>,
    #[asset(path = "textures/pills/invincibility_pill.png")]
    pub invincibility_pill: Handle<Image>,
}
