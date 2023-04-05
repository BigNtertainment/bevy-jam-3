use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(debug_spawn);
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_xyz(0., 0., 0.),
                sprite: Sprite {
                    color: Color::hex("FF0000").unwrap(),
                    custom_size: Some(Vec2::splat(24.)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

/// debug_spawn spawns test enemy somwehere on the map.
fn debug_spawn(mut commands: Commands) {
    commands.spawn(EnemyBundle::default());
}
