use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::{FieldValue, LdtkEntity};
use bevy_rapier2d::prelude::{ActiveCollisionTypes, Collider, RigidBody, Sensor};
use rand::seq::IteratorRandom;

use crate::{cleanup::cleanup, loading::TextureAssets, GameState, WorldState};

pub struct PillPlugin;

impl Plugin for PillPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Pill>()
            .add_systems((update_pill_texture, adjust_pill_scale).in_set(OnUpdate(GameState::Playing)))
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
    Vulnerability { amount: f32, duration: Duration },
}

impl PillEffect {
    pub fn positive() -> Vec<Self> {
        vec![
            Self::Heal { amount: 15. },
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
            Self::Vulnerability {
                amount: 2.0,
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
pub struct PillBundle {
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

impl LdtkEntity for PillBundle {
    fn bundle_entity(
        entity_instance: &bevy_ecs_ldtk::EntityInstance,
        _layer_instance: &bevy_ecs_ldtk::prelude::LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&bevy_ecs_ldtk::prelude::TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let mut pill_effect_str = "".to_string();

        for field in &entity_instance.field_instances {
            match field.identifier.as_str() {
                "Pill_Type" => {
                    if let FieldValue::Enum(enum_value) = &field.value {
                        pill_effect_str = enum_value.clone().expect("Empty pill type");
                    } else {
                        panic!("Pill_Type field is not an enum value");
                    }
                }
                other => panic!("Unknown pill field: {}", other),
            }
        }

        let pill_effect = match pill_effect_str.as_str() {
            "Heal" => PillEffect::positive()[0],
            "Speed" => PillEffect::positive()[1],
            "ToxicFart" => PillEffect::positive()[2],
            "Invisibility" => PillEffect::positive()[3],
            "Invincibility" => PillEffect::positive()[4],
            other => panic!("Unknown pill effect: {}", other),
        };

        Self {
            pill: Pill::new(pill_effect),
            ..default()
        }
    }
}

fn adjust_pill_scale(mut query: Query<&mut Transform, Added<Pill>>) {
    for mut transform in query.iter_mut() {
        transform.scale = Vec2::splat(0.25).extend(1.);
    }
}

fn update_pill_texture(
    mut query: Query<(&Pill, &mut Handle<Image>)>,
    textures: Res<TextureAssets>,
) {
    for (pill, mut texture) in query.iter_mut() {
        *texture = pill.get_texture(&textures);
    }
}
