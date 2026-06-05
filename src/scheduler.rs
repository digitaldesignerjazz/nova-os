/// Emotional + Self-Improving Scheduler for Nova OS
///
/// Includes multi-hop emotional propagation with relationship-aware influence
/// and a more developed emotional memory system.

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
pub struct EmotionalMemory {
    pub state: EmotionalState,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct Task {
    pub id: u64,
    pub priority: u8,
    pub emotional_state: EmotionalState,
    pub memory: [Option<EmotionalMemory>; 4],
    pub memory_index: usize,
}

impl Task {
    /// Record a new emotional state in the circular history buffer
    pub fn record_emotion(&mut self, state: EmotionalState, time: u64) {
        self.memory[self.memory_index] = Some(EmotionalMemory { state, timestamp: time });
        self.memory_index = (self.memory_index + 1) % self.memory.len();
    }

    /// Returns the 4 most recent emotional states (newest first)
    pub fn get_recent_emotions(&self) -> [Option<EmotionalMemory>; 4] {
        let mut result = [None; 4];
        for i in 0..4 {
            let idx = (self.memory_index + 4 - i - 1) % 4;
            result[i] = self.memory[idx];
        }
        result
    }

    /// Get the most recent emotional state recorded
    pub fn get_last_emotional_state(&self) -> Option<EmotionalState> {
        for i in 0..self.memory.len() {
            let idx = (self.memory_index + self.memory.len() - i - 1) % self.memory.len();
            if let Some(mem) = self.memory[idx] {
                return Some(mem.state);
            }
        }
        None
    }

    /// Check if this task was in a stressed state recently
    pub fn was_recently_stressed(&self, lookback: usize) -> bool {
        let lookback = lookback.min(self.memory.len());
        for i in 0..lookback {
            let idx = (self.memory_index + self.memory.len() - i - 1) % self.memory.len();
            if let Some(mem) = self.memory[idx] {
                if mem.state == EmotionalState::Stressed {
                    return true;
                }
            }
        }
        false
    }

    /// Count how many times a specific emotional state appeared in recent history
    pub fn count_recent_state(&self, state: EmotionalState, lookback: usize) -> usize {
        let lookback = lookback.min(self.memory.len());
        let mut count = 0;
        for i in 0..lookback {
            let idx = (self.memory_index + self.memory.len() - i - 1) % self.memory.len();
            if let Some(mem) = self.memory[idx] {
                if mem.state == state {
                    count += 1;
                }
            }
        }
        count
    }

    /// Returns true if the task's emotional state has been unstable recently
    pub fn is_emotionally_unstable(&self, lookback: usize) -> bool {
        self.count_recent_state(EmotionalState::Stressed, lookback) >= 2
    }
}

pub struct SelfImprovingScheduler {
    next_task_id: AtomicU64,
    time: u64,
}

impl SelfImprovingScheduler {
    pub const fn new() -> Self {
        SelfImprovingScheduler {
            next_task_id: AtomicU64::new(0),
            time: 0,
        }
    }

    pub fn create_task(&self, base_priority: u8) -> Task {
        let id = self.next_task_id.fetch_add(1, Ordering::Relaxed);
        Task {
            id,
            priority: base_priority,
            emotional_state: EmotionalState::Neutral,
            memory: [None; 4],
            memory_index: 0,
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

        task.record_emotion(task.emotional_state, self.time);
        self.time += 1;

        if task.emotional_state != previous_state {
            println!("Task {} changed: {:?} -> {:?}", task.id, previous_state, task.emotional_state);
        }
    }

    /// Multi-hop propagation that respects recent emotional memory
    pub fn propagate_emotion_multi_hop(
        &self,
        source: &Task,
        all_tasks: &mut [Task],
        max_hops: usize,
    ) {
        if max_hops == 0 || all_tasks.is_empty() {
            return;
        }

        self.propagate_emotion_single(source, all_tasks);

        let mut current_sources: Vec<usize> = vec![source.id as usize];

        for hop in 1..=max_hops {
            let mut new_influenced = Vec::new();

            for &src_id in &current_sources {
                if let Some(src_task) = all_tasks.iter().find(|t| t.id as usize == src_id) {
                    let base_influence = self.calculate_influence_strength(&src_task.emotional_state);

                    if base_influence == 0 {
                        continue;
                    }

                    for target in all_tasks.iter_mut() {
                        if target.id as usize == src_id {
                            continue;
                        }

                        // Use emotional memory: recently stressed targets resist influence
                        let resistance = if target.was_recently_stressed(2) || target.is_emotionally_unstable(3) {
                            0.5
                        } else {
                            1.0
                        };

                        let relationship = self.get_relationship_strength(source.id, target.id);
                        let effective = self.compute_effective_influence(base_influence, relationship);
                        let final_influence = (effective as f32 * resistance) as i8;

                        if final_influence != 0 {
                            let new_state = self.compute_propagated_state(
                                &src_task.emotional_state,
                                &target.emotional_state,
                                final_influence,
                            );

                            if new_state != target.emotional_state {
                                println!(
                                    "[Hop {}] Emotional influence: Task {} -> Task {} ({:?} -> {:?})",
                                    hop, src_task.id, target.id, target.emotional_state, new_state
                                );
                                target.emotional_state = new_state;
                                target.record_emotion(new_state, self.time);
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

    fn propagate_emotion_single(&self, source: &Task, targets: &mut [Task]) {
        let base_influence = self.calculate_influence_strength(&source.emotional_state);
        if base_influence == 0 {
            return;
        }

        for target in targets.iter_mut() {
            if target.id == source.id {
                continue;
            }

            let relationship = self.get_relationship_strength(source.id, target.id);
            let effective = self.compute_effective_influence(base_influence, relationship);

            // Memory-based resistance
            let resistance = if target.was_recently_stressed(2) || target.is_emotionally_unstable(3) {
                0.55
            } else {
                1.0
            };

            let final_influence = (effective as f32 * resistance) as i8;

            if final_influence != 0 {
                let new_state = self.compute_propagated_state(
                    &source.emotional_state,
                    &target.emotional_state,
                    final_influence,
                );

                if new_state != target.emotional_state {
                    println!(
                        "Emotional propagation: Task {} influenced Task {} -> {:?}",
                        source.id, target.id, new_state
                    );
                    target.emotional_state = new_state;
                    target.record_emotion(new_state, self.time);
                }
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

    fn get_relationship_strength(&self, _from_id: u64, _to_id: u64) -> i8 {
        1
    }

    fn compute_effective_influence(&self, base_influence: i8, relationship: i8) -> i8 {
        if relationship == 0 {
            return 0;
        }
        let scaled = base_influence as i16 * relationship as i16;
        scaled.clamp(-4, 4) as i8
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
// - Replace placeholder relationship system with real RelationshipGraph
// - Use full emotional memory (not just stress) for smarter resistance
// - Add emotional decay / fading of old memories
