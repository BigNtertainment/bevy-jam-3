use bevy::{ecs::schedule::SystemConfigs, prelude::*};
use drug_test_proc_macros::Temporary;

use crate::GameState;

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MovementBoost>()
            .register_type::<Invisibility>()
            .register_type::<Invincibility>()
            .register_type::<Dizziness>()
            .add_systems(effect_systems::<MovementBoost>())
            .add_systems(effect_systems::<Invisibility>())
            .add_systems(effect_systems::<Invincibility>())
            .add_systems(effect_systems::<Dizziness>())
            .add_system(invisibility_vfx.in_set(OnUpdate(GameState::Playing)))
            .add_system(reset_invisibility_vfx.in_set(OnUpdate(GameState::Playing)));
    }
}

pub trait Temporary {
    fn get_timer(&mut self) -> &mut Timer;
}

fn effect_systems<Effect: Temporary + Component>() -> SystemConfigs {
    (update_effect::<Effect>,).in_set(OnUpdate(GameState::Playing))
}

#[derive(Reflect, Component, Clone, Default, Debug, Temporary)]
#[reflect(Component)]
pub struct MovementBoost {
    pub multiplier: f32,
    pub timer: Timer,
}

#[derive(Reflect, Component, Clone, Default, Debug, Temporary)]
#[reflect(Component)]
pub struct Invisibility {
    pub timer: Timer,
}

#[derive(Reflect, Component, Clone, Default, Debug, Temporary)]
#[reflect(Component)]
pub struct Invincibility {
    pub timer: Timer,
}

#[derive(Reflect, Component, Clone, Default, Debug, Temporary)]
#[reflect(Component)]
pub struct Dizziness {
    pub timer: Timer,
}

fn update_effect<Effect: Temporary + Component>(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Effect)>,
    time: Res<Time>,
) {
    for (entity, mut effect) in query.iter_mut() {
        if effect.get_timer().tick(time.delta()).just_finished() {
            commands.entity(entity).remove::<Effect>();
        }
    }
}

fn invisibility_vfx(mut invisible_query: Query<&mut Sprite, Added<Invisibility>>) {
    for mut sprite in invisible_query.iter_mut() {
        sprite.color.set_a(0.5);
    }
}

fn reset_invisibility_vfx(
    mut invisible_query: Query<&mut Sprite>,
    mut invisibility_removals: RemovedComponents<Invisibility>,
) {
    for entity in invisibility_removals.iter() {
        if let Ok(mut sprite) = invisible_query.get_mut(entity) {
            sprite.color.set_a(1.0);
        }
    }
}
