use bevy::prelude::*;

use crate::actions::game_control::{get_movement, GameControl};
use crate::GameState;

mod game_control;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>()
            .add_event::<BurstActions>()
            .add_systems(
                (set_movement_actions, register_burst_actions).in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BurstActions {
    Punch,
    ConsumePill { index: usize },
}

pub fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    actions.player_movement = Vec2::new(
        get_movement(GameControl::Right, &keyboard_input)
            - get_movement(GameControl::Left, &keyboard_input),
        get_movement(GameControl::Up, &keyboard_input)
            - get_movement(GameControl::Down, &keyboard_input),
    );
}

pub fn register_burst_actions(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut burst_actions: EventWriter<BurstActions>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) || keyboard_input.just_pressed(KeyCode::Space) {
        burst_actions.send(BurstActions::Punch);
    }

    if keyboard_input.just_pressed(KeyCode::Key1) {
        burst_actions.send(BurstActions::ConsumePill { index: 0 });
    }

    if keyboard_input.just_pressed(KeyCode::Key2) {
        burst_actions.send(BurstActions::ConsumePill { index: 1 });
    }

    if keyboard_input.just_pressed(KeyCode::Key3) {
        burst_actions.send(BurstActions::ConsumePill { index: 2 });
    }
}
