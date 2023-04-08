use bevy::{
    prelude::*,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, AsBindGroup, ShaderRef,
        },
        texture::BevyDefault,
        view::RenderLayers, camera::RenderTarget,
    },
    sprite::{Material2dPlugin, Material2d, MaterialMesh2dBundle}, reflect::TypeUuid, core_pipeline::clear_color::ClearColorConfig,
};

use crate::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(camera_setup.in_schedule(OnEnter(GameState::Loading)));
        
    }
}

fn camera_setup(
    mut commands: Commands,
) {
    // This assumes we only have a single window
    commands.spawn(Camera2dBundle::default());

}
