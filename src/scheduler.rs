/// Emotional + Self-Improving Scheduler for Nova OS
///
/// Core of Nova OS's emotional swarm intelligence.

use core::sync::atomic::{AtomicU64, Ordering};

/// Emotional states for tasks / agents.
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
            println!(
                "Task {} emotional change: {:?} -> {:?}",
                task.id, previous_state, task.emotional_state
            );
        }
    }

    /// Propagate emotional state from one task to others.
    /// This is a core mechanism for emotional swarm behavior.
    pub fn propagate_emotion(&self, source: &Task, targets: &mut [Task]) {
        if targets.is_empty() {
            return;
        }

        let influence_strength = match source.emotional_state {
            EmotionalState::Focused | EmotionalState::Loyal => 2,
            EmotionalState::Excited => 1,
            EmotionalState::Content => 1,
            EmotionalState::Curious => 1,
            EmotionalState::Stressed => -1, // Negative spread (can be tuned)
            EmotionalState::Neutral => 0,
        };

        if influence_strength == 0 {
            return;
        }

        for target in targets.iter_mut() {
            if target.id == source.id {
                continue; // Don't influence self
            }

            let new_state = match (source.emotional_state, target.emotional_state) {
                // Positive propagation
                (EmotionalState::Focused, EmotionalState::Neutral) => EmotionalState::Content,
                (EmotionalState::Loyal, _) => EmotionalState::Loyal,
                (EmotionalState::Excited, EmotionalState::Neutral | EmotionalState::Content) => EmotionalState::Excited,

                // Curious can spread exploration
                (EmotionalState::Curious, EmotionalState::Neutral) => EmotionalState::Curious,

                // Negative spread (Stressed)
                (EmotionalState::Stressed, EmotionalState::Content | EmotionalState::Neutral) => EmotionalState::Stressed,

                _ => target.emotional_state,
            };

            if new_state != target.emotional_state {
                println!(
                    "Emotional propagation: Task {} ({:?}) influenced Task {} -> {:?}",
                    source.id, source.emotional_state, target.id, new_state
                );
                target.emotional_state = new_state;
            }
        }
    }
}

// TODO:
// - Add relationship / loyalty graph between tasks
// - Make propagation strength depend on relationship closeness
// - Support swarm-level emotional consensus
// - Persistent emotional memory
