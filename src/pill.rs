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
            .add_system(update_pill_color.in_set(OnUpdate(GameState::Playing)))
            .add_system(cleanup::<Pill>.in_schedule(OnExit(WorldState::Yes)));
    }
}

#[derive(Reflect, Debug, Copy, Clone, PartialEq)]
pub enum PillEffect {
    Heal { amount: f32 },
    Speed { amount: f32 },
    ToxicFart,
    Invisibility { duration: Duration },
    Invincibility { duration: Duration },
    Sneeze,
    Dizziness,
    Blindness,
}

impl PillEffect {
    pub fn positive() -> Vec<Self> {
        vec![
            Self::Heal { amount: 10. },
            Self::Speed { amount: 1.5 },
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
            Self::Speed { amount: 0.5 },
            Self::Sneeze,
            Self::Dizziness,
            Self::Blindness,
        ]
    }
}

#[derive(Component, Reflect, Debug, Copy, Clone, PartialEq)]
#[reflect(Component)]
pub struct Pill {
    main_effect: PillEffect,
    side_effect: PillEffect,
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
}

impl Default for Pill {
    fn default() -> Self {
        Self {
            main_effect: PillEffect::positive()[0],
            side_effect: PillEffect::negative()[0],
        }
    }
}

impl From<Pill> for Color {
    fn from(pill: Pill) -> Self {
        match pill.main_effect {
            PillEffect::Heal { .. } => Color::hex("#d62004").unwrap(),
            PillEffect::Speed { .. } => Color::hex("#04b3d6").unwrap(),
            PillEffect::ToxicFart => Color::hex("#419e08").unwrap(),
            PillEffect::Invisibility { .. } => Color::hex("#b50eed").unwrap(),
            PillEffect::Invincibility { .. } => Color::hex("#120eed").unwrap(),
            _ => Color::hex("#000000").unwrap(),
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
            collider: Collider::ball(16.),
            sensor: Sensor,
            active_collision_types: ActiveCollisionTypes::all(),
        }
    }
}

fn pill_setup(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn(PillBundle {
        pill: Pill::new(PillEffect::positive()[0]),
        sprite_bundle: SpriteBundle {
            texture: textures.pill.clone(),
            transform: Transform::from_translation(Vec3::new(200., 0., 1.)),
            ..Default::default()
        },
        ..default()
    });

    commands.spawn(PillBundle {
        pill: Pill::new(PillEffect::positive()[1]),
        sprite_bundle: SpriteBundle {
            texture: textures.pill.clone(),
            transform: Transform::from_translation(Vec3::new(-150., 125., 1.)),
            ..Default::default()
        },
        ..default()
    });

    commands.spawn(PillBundle {
        pill: Pill::new(PillEffect::positive()[2]),
        sprite_bundle: SpriteBundle {
            texture: textures.pill.clone(),
            transform: Transform::from_translation(Vec3::new(-140., -20., 1.)),
            ..Default::default()
        },
        ..default()
    });

    commands.spawn(PillBundle {
        pill: Pill::new(PillEffect::positive()[3]),
        sprite_bundle: SpriteBundle {
            texture: textures.pill.clone(),
            transform: Transform::from_translation(Vec3::new(140., -45., 1.)),
            ..Default::default()
        },
        ..default()
    });
}

fn update_pill_color(mut query: Query<(&Pill, &mut Sprite)>) {
    for (pill, mut sprite) in query.iter_mut() {
        sprite.color = Color::from(*pill);
    }
}
