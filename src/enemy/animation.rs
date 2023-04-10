use std::time::Duration;

use bevy_spritesheet_animation::{animation_manager::AnimationManager, animation_graph::{AnimationTransitionCondition, AnimationTransitionMode}, animation::{Animation, AnimationBounds}};

pub fn enemy_animation_manager() -> AnimationManager {
	let mut animation_manager = AnimationManager::new(
		vec![
			// Idle
			Animation::new(AnimationBounds::new(0, 0), Duration::from_millis(500)),
			// Walking
			Animation::new(AnimationBounds::new(0, 19), Duration::from_millis(80)),
			// Stun
			Animation::new(AnimationBounds::new(20, 21), Duration::from_millis(350)),
			// Shooting
			Animation::new(AnimationBounds::new(22, 41), Duration::from_millis(20)),
		],
		0,
	);

	animation_manager.add_state("walk".to_string(), false);
	animation_manager.add_state("stun".to_string(), false);
	animation_manager.add_state("shoot".to_string(), false);

	animation_manager.add_graph_edge(
		0,
		1,
		AnimationTransitionCondition::new(Box::new(|state| state["walk"]))
			.with_mode(AnimationTransitionMode::Immediate),
	);
	animation_manager.add_graph_edge(
		1,
		0,
		AnimationTransitionCondition::new(Box::new(|state| !state["walk"]))
			.with_mode(AnimationTransitionMode::Immediate),
	);
	animation_manager.add_graph_edge(
		1,
		1,
		AnimationTransitionCondition::new(Box::new(|state| state["walk"])),
	);

	animation_manager.add_graph_edge(
		0,
		2,
		AnimationTransitionCondition::new(Box::new(|state| state["stun"]))
			.with_mode(AnimationTransitionMode::Immediate),
	);
	animation_manager.add_graph_edge(
		1,
		2,
		AnimationTransitionCondition::new(Box::new(|state| state["stun"]))
			.with_mode(AnimationTransitionMode::Immediate),
	);
	animation_manager.add_graph_edge(
		2,
		2,
		AnimationTransitionCondition::new(Box::new(|state| state["stun"])),
	);
	animation_manager.add_graph_edge(
		2,
		0,
		AnimationTransitionCondition::new(Box::new(|state| !state["stun"]))
			.with_mode(AnimationTransitionMode::Immediate),
	);

	animation_manager.add_graph_edge(
		0,
		3,
		AnimationTransitionCondition::new(Box::new(|state| state["shoot"]))
			.with_mode(AnimationTransitionMode::Immediate),
	);
	animation_manager.add_graph_edge(
		1,
		3,
		AnimationTransitionCondition::new(Box::new(|state| state["shoot"]))
			.with_mode(AnimationTransitionMode::Immediate),
	);
	animation_manager.add_graph_edge(
		3,
		0,
		AnimationTransitionCondition::new(Box::new(|_| true)),
	);

	animation_manager
}