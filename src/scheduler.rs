/// Emotional + Self-Improving Scheduler for Nova OS
///
/// Includes multi-hop emotional propagation and basic relationship tracking.

use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmotionalState {
    Neutral,
    Focused,
    Loyal,
    Curious,
    Stressed,
    Content,
    Excited,
}

impl EmotionalState {
    pub fn is_positive(&self) -> bool {
        matches!(self, Self::Focused | Self::Loyal | Self::Content | Self::Excited)
    }

    pub fn priority_modifier(&self) -> i8 {
        match self {
            Self::Focused => 3,
            Self::Loyal => 2,
            Self::Excited => 1,
            Self::Content => 0,
            Self::Curious => 0,
            Self::Neutral => 0,
            Self::Stressed => -2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Task {
    pub id: u64,
    pub priority: u8,
    pub emotional_state: EmotionalState,
}

pub struct SelfImprovingScheduler {
    next_task_id: AtomicU64,
}

impl SelfImprovingScheduler {
    pub const fn new() -> Self {
        SelfImprovingScheduler {
            next_task_id: AtomicU64::new(0),
        }
    }

    pub fn create_task(&self, base_priority: u8) -> Task {
        let id = self.next_task_id.fetch_add(1, Ordering::Relaxed);
        Task {
            id,
            priority: base_priority,
            emotional_state: EmotionalState::Neutral,
        }
    }

    pub fn schedule(&self, tasks: &mut [Task]) {
        tasks.sort_by_key(|t| {
            let emotional_boost = t.emotional_state.priority_modifier() as i16;
            let final_priority = (t.priority as i16 + emotional_boost).clamp(0, 255) as u8;
            core::cmp::Reverse(final_priority)
        });
    }

    pub fn collect_feedback(&mut self, task: &mut Task, success: bool, latency_ms: u64) {
        let previous_state = task.emotional_state;

        task.emotional_state = match (success, latency_ms, task.emotional_state) {
            (true, latency, _) if latency < 30 => EmotionalState::Focused,
            (true, latency, EmotionalState::Loyal) if latency < 80 => EmotionalState::Loyal,
            (true, latency, _) if latency > 150 => EmotionalState::Curious,
            (false, _, _) => EmotionalState::Stressed,
            (true, _, EmotionalState::Stressed) => EmotionalState::Content,
            (true, _, EmotionalState::Neutral) => EmotionalState::Content,
            (true, _, state) => state,
            (false, _, EmotionalState::Focused) => EmotionalState::Content,
            _ => task.emotional_state,
        };

        if task.emotional_state != previous_state {
            println!("Task {} changed: {:?} -> {:?}", task.id, previous_state, task.emotional_state);
        }
    }

    /// Propagate emotional state with support for multiple hops.
    /// `max_hops` controls how far the emotion can spread through the group.
    pub fn propagate_emotion_multi_hop(
        &self,
        source: &Task,
        all_tasks: &mut [Task],
        max_hops: usize,
    ) {
        if max_hops == 0 || all_tasks.is_empty() {
            return;
        }

        // Start with direct influence (hop 0)
        self.propagate_emotion_single(source, all_tasks);

        // Multi-hop propagation with diminishing influence
        let mut current_sources: Vec<usize> = vec![source.id as usize];

        for hop in 1..=max_hops {
            let mut new_influenced = Vec::new();

            for &src_id in &current_sources {
                // Find the current state of this source (it may have changed)
                if let Some(src_task) = all_tasks.iter().find(|t| t.id as usize == src_id) {
                    let influence = self.calculate_influence_strength(&src_task.emotional_state);

                    if influence == 0 {
                        continue;
                    }

                    for target in all_tasks.iter_mut() {
                        if target.id as usize == src_id {
                            continue;
                        }

                        // Simple relationship model: assume uniform relationship for now
                        // In future this can be replaced with a real relationship graph
                        let relationship_strength = self.get_relationship_strength(src_id as u64, target.id);

                        let effective_influence = influence * relationship_strength;

                        if effective_influence > 0 {
                            let new_state = self.compute_propagated_state(
                                &src_task.emotional_state,
                                &target.emotional_state,
                                effective_influence,
                            );

                            if new_state != target.emotional_state {
                                println!(
                                    "[Hop {}] Emotional influence: Task {} ({:?}) -> Task {} ({:?} -> {:?}) [rel={:.1}]",
                                    hop,
                                    src_task.id,
                                    src_task.emotional_state,
                                    target.id,
                                    target.emotional_state,
                                    new_state,
                                    relationship_strength
                                );
                                target.emotional_state = new_state;
                                new_influenced.push(target.id as usize);
                            }
                        }
                    }
                }
            }

            current_sources = new_influenced;
            if current_sources.is_empty() {
                break;
            }
        }
    }

    /// Single-hop propagation (used internally)
    fn propagate_emotion_single(&self, source: &Task, targets: &mut [Task]) {
        let influence = self.calculate_influence_strength(&source.emotional_state);
        if influence == 0 {
            return;
        }

        for target in targets.iter_mut() {
            if target.id == source.id {
                continue;
            }

            let relationship = self.get_relationship_strength(source.id, target.id);
            let effective = influence * relationship;

            let new_state = self.compute_propagated_state(
                &source.emotional_state,
                &target.emotional_state,
                effective,
            );

            if new_state != target.emotional_state {
                println!(
                    "Emotional propagation: Task {} ({:?}) influenced Task {} -> {:?}",
                    source.id, source.emotional_state, target.id, new_state
                );
                target.emotional_state = new_state;
            }
        }
    }

    fn calculate_influence_strength(&self, state: &EmotionalState) -> i8 {
        match state {
            EmotionalState::Focused | EmotionalState::Loyal => 2,
            EmotionalState::Excited | EmotionalState::Curious => 1,
            EmotionalState::Content => 1,
            EmotionalState::Stressed => -1,
            EmotionalState::Neutral => 0,
        }
    }

    /// Basic relationship model.
    /// Currently returns a default strength. Can be extended with a real graph.
    fn get_relationship_strength(&self, _from_id: u64, _to_id: u64) -> f32 {
        // TODO: Replace with actual relationship tracking / loyalty graph
        // For now we use a default relationship strength of 1.0
        1.0
    }

    fn compute_propagated_state(
        &self,
        source_state: &EmotionalState,
        target_state: &EmotionalState,
        influence: i8,
    ) -> EmotionalState {
        if influence <= 0 {
            return *target_state;
        }

        match (source_state, target_state) {
            (EmotionalState::Focused, EmotionalState::Neutral) => EmotionalState::Content,
            (EmotionalState::Loyal, _) => EmotionalState::Loyal,
            (EmotionalState::Excited, EmotionalState::Neutral | EmotionalState::Content) => EmotionalState::Excited,
            (EmotionalState::Curious, EmotionalState::Neutral) => EmotionalState::Curious,
            (EmotionalState::Stressed, EmotionalState::Content | EmotionalState::Neutral) => EmotionalState::Stressed,
            _ => *target_state,
        }
    }
}

// TODO:
// - Implement real relationship / loyalty graph between tasks
// - Make get_relationship_strength use actual data
// - Add emotional memory / history
// - Swarm-level emotional consensus mechanisms
