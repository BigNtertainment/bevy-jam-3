use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::LdtkAsset;
use bevy_kira_audio::AudioSource;

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
        .add_collection_to_loading_state::<_, LevelAssets>(GameState::Loading)
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
pub struct AudioAssets {
    #[asset(path = "audio/death.ogg")]
    pub death: Handle<AudioSource>,
    #[asset(path = "audio/fart.ogg")]
    pub fart: Handle<AudioSource>,
    #[asset(path = "audio/hit.ogg")]
    pub punch: Handle<AudioSource>,
    #[asset(path = "audio/notice.ogg")]
    pub notice: Handle<AudioSource>,
    #[asset(path = "audio/shot.ogg")]
    pub shot: Handle<AudioSource>,
    #[asset(path = "audio/sneeze.ogg")]
    pub sneeze: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 128., columns = 5, rows = 10))]
    #[asset(path = "textures/player/player-up.png")]
    pub player_up: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 128., columns = 5, rows = 10))]
    #[asset(path = "textures/player/player-down.png")]
    pub player_down: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 128., columns = 5, rows = 10))]
    #[asset(path = "textures/player/player-left.png")]
    pub player_left: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 128., columns = 5, rows = 10))]
    #[asset(path = "textures/player/player-right.png")]
    pub player_right: Handle<TextureAtlas>,
    #[asset(path = "textures/player/player-body.png")]
    pub player_body: Handle<Image>,
    #[asset(path = "textures/wall.png")]
    pub wall: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 200., tile_size_y = 260., columns = 5, rows = 9))]
    #[asset(path = "textures/enemy/enemy-up.png")]
    pub enemy_up: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 200., tile_size_y = 300., columns = 5, rows = 9))]
    #[asset(path = "textures/enemy/enemy-down.png")]
    pub enemy_down: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 640., tile_size_y = 480., columns = 5, rows = 10))]
    #[asset(path = "textures/enemy/enemy-left.png")]
    pub enemy_left: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 640., tile_size_y = 480., columns = 5, rows = 10))]
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

#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
    #[asset(path = "ldtk/level1.ldtk")]
    pub ldtk_handle: Handle<LdtkAsset>,
}
