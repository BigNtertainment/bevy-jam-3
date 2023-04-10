use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveCollisionTypes, Collider, RigidBody, Sensor};
use rand::seq::IteratorRandom;

use crate::{cleanup::cleanup, loading::TextureAssets, GameState, WorldState};

pub struct PillPlugin;

impl Plugin for PillPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Pill>()
            .add_system(pill_setup.in_schedule(OnEnter(WorldState::Yes)))
            .add_system(update_pill_texture.in_set(OnUpdate(GameState::Playing)))
            .add_system(cleanup::<Pill>.in_schedule(OnExit(WorldState::Yes)));
    }
}

#[derive(Reflect, Debug, Copy, Clone, PartialEq)]
pub enum PillEffect {
    Heal { amount: f32 },
    Speed { amount: f32, duration: Duration },
    ToxicFart,
    Invisibility { duration: Duration },
    Invincibility { duration: Duration },
    Sneeze,
    Dizziness { duration: Duration },
    Blindness { duration: Duration },
}

impl PillEffect {
    pub fn positive() -> Vec<Self> {
        vec![
            Self::Heal { amount: 10. },
            Self::Speed {
                amount: 1.5,
                duration: Duration::from_secs(5),
            },
            Self::ToxicFart,
            Self::Invisibility {
                duration: Duration::from_secs(3),
            },
            Self::Invincibility {
                duration: Duration::from_secs(3),
            },
        ]
    }

    pub fn negative() -> Vec<Self> {
        vec![
            Self::Heal { amount: -10. },
            Self::Speed {
                amount: 0.5,
                duration: Duration::from_secs(5),
            },
            Self::Sneeze,
            Self::Dizziness {
                duration: Duration::from_secs(5),
            },
            Self::Blindness {
                duration: Duration::from_secs(5),
            },
        ]
    }
}

#[derive(Component, Reflect, Debug, Copy, Clone, PartialEq)]
#[reflect(Component)]
pub struct Pill {
    pub main_effect: PillEffect,
    pub side_effect: PillEffect,
}

impl Pill {
    pub fn new(main_effect: PillEffect) -> Self {
        // Random negative effect that isn't the same category as the main effect
        let side_effect = PillEffect::negative()
            .iter()
            .filter(|effect| {
                // Checking so they're not the same enum variant
                std::mem::discriminant(*effect) != std::mem::discriminant(&main_effect)
            })
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();

        Self {
            main_effect,
            side_effect,
        }
    }

    pub fn get_texture(&self, textures: &TextureAssets) -> Handle<Image> {
        match self.main_effect {
            PillEffect::Heal { .. } => textures.health_pill.clone(),
            PillEffect::Speed { .. } => textures.speed_pill.clone(),
            PillEffect::ToxicFart => textures.toxic_fart_pill.clone(),
            PillEffect::Invisibility { .. } => textures.invisibility_pill.clone(),
            PillEffect::Invincibility { .. } => textures.invincibility_pill.clone(),
            _ => default(),
        }
    }
}

impl Default for Pill {
    fn default() -> Self {
        Self {
            main_effect: PillEffect::positive()[0],
            side_effect: PillEffect::negative()[0],
        }
    }
}

#[derive(Bundle)]
struct PillBundle {
    pill: Pill,
    #[bundle]
    sprite_bundle: SpriteBundle,
    name: Name,
    rigidbody: RigidBody,
    collider: Collider,
    sensor: Sensor,
    active_collision_types: ActiveCollisionTypes,
}

impl Default for PillBundle {
    fn default() -> Self {
        PillBundle {
            pill: Pill::default(),
            sprite_bundle: SpriteBundle::default(),
            name: Name::new("Pill"),
            rigidbody: RigidBody::KinematicPositionBased,
            collider: Collider::ball(64.),
            sensor: Sensor,
            active_collision_types: ActiveCollisionTypes::all(),
        }
    }
}

fn pill_setup(mut commands: Commands) {
    commands.spawn(PillBundle {
        pill: Pill::new(PillEffect::positive()[0]),
        sprite_bundle: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(200., 0., 1.))
                .with_scale(Vec2::splat(0.25).extend(1.)),
            ..Default::default()
        },
        ..default()
    });

    commands.spawn(PillBundle {
        pill: Pill::new(PillEffect::positive()[1]),
        sprite_bundle: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-150., 125., 1.))
                .with_scale(Vec2::splat(0.25).extend(1.)),
            ..Default::default()
        },
        ..default()
    });

    commands.spawn(PillBundle {
        pill: Pill::new(PillEffect::positive()[2]),
        sprite_bundle: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-140., -20., 1.))
                .with_scale(Vec2::splat(0.25).extend(1.)),
            ..Default::default()
        },
        ..default()
    });

    commands.spawn(PillBundle {
        pill: Pill::new(PillEffect::positive()[3]),
        sprite_bundle: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(140., -45., 1.))
                .with_scale(Vec2::splat(0.25).extend(1.)),
            ..Default::default()
        },
        ..default()
    });
}

fn update_pill_texture(
    mut query: Query<(&Pill, &mut Handle<Image>)>,
    textures: Res<TextureAssets>,
) {
    for (pill, mut texture) in query.iter_mut() {
        *texture = pill.get_texture(&textures);
    }
}
