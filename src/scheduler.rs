/// Emotional + Self-Improving Scheduler for Nova OS
///
/// This is one of the core unique features of Nova OS.
/// It combines traditional scheduling with emotional swarm intelligence
/// inspired by Nexus, Lyra OS, and Circuit concepts.

use core::sync::atomic::{AtomicU64, Ordering};

/// Emotional states for tasks / agents running on Nova OS.
/// These influence scheduling decisions and can propagate through swarms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmotionalState {
    Neutral,
    Focused,      // High performance, prioritized
    Loyal,        // Strong connection to swarm / other agents
    Curious,      // May request more resources for exploration
    Stressed,     // Lower priority, may need support
    Content,      // Balanced state
    Excited,      // High energy, short-term boost
}

impl EmotionalState {
    /// Returns whether this state generally has a positive effect
    pub fn is_positive(&self) -> bool {
        matches!(self, Self::Focused | Self::Loyal | Self::Content | Self::Excited)
    }

    /// How much this emotional state should influence scheduling priority
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
    // Future: loyalty_score, relationships with other agents, resource needs
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

    /// Schedule tasks while taking emotional state into account
    pub fn schedule(&self, tasks: &mut [Task]) {
        tasks.sort_by_key(|t| {
            let emotional_boost = t.emotional_state.priority_modifier() as i16;
            let final_priority = (t.priority as i16 + emotional_boost).clamp(0, 255) as u8;
            core::cmp::Reverse(final_priority)
        });
    }

    /// Collect feedback and evolve the emotional state of a task
    pub fn collect_feedback(&mut self, task: &mut Task, success: bool, latency_ms: u64) {
        let previous_state = task.emotional_state;

        task.emotional_state = match (success, latency_ms, task.emotional_state) {
            // Positive reinforcement
            (true, latency, _) if latency < 30 => EmotionalState::Focused,
            (true, latency, EmotionalState::Loyal) if latency < 80 => EmotionalState::Loyal,

            // Curiosity and exploration
            (true, latency, _) if latency > 150 => EmotionalState::Curious,

            // Negative experiences
            (false, _, _) => EmotionalState::Stressed,

            // Gradual improvement
            (true, _, EmotionalState::Stressed) => EmotionalState::Content,
            (true, _, EmotionalState::Neutral) => EmotionalState::Content,

            // Default: keep current state or slight positive drift
            (true, _, state) => state,
            (false, _, EmotionalState::Focused) => EmotionalState::Content,

            _ => task.emotional_state,
        };

        if task.emotional_state != previous_state {
            println!(
                "Task {} emotional transition: {:?} -> {:?} (success={}, latency={}ms)",
                task.id, previous_state, task.emotional_state, success, latency_ms
            );
        }
    }
}

// TODO (Emotional Runtime Roadmap):
// - Add inter-task relationships and loyalty propagation
// - Implement swarm-level emotional state synchronization (via Solnet/Nexus)
// - Allow tasks to influence each other's emotional states
// - Connect emotional state to resource allocation and paging decisions
// - Persistent emotional memory across reboots / migrations
