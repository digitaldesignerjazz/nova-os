/// SwarmRuntime for Nova OS
///
/// Higher-level abstraction for managing groups of emotional tasks/agents.
/// Includes relationship graph for emotional propagation.

use crate::scheduler::{SelfImprovingScheduler, Task, EmotionalState};

/// Simple fixed-size relationship graph for early kernel development.
/// Stores relationship strength between tasks (i8).
/// Positive = friendly/loyal, Negative = adversarial.
pub struct RelationshipGraph {
    /// relationships[from][to]
    relationships: [[i8; 16]; 16],
    size: usize,
}

impl RelationshipGraph {
    pub const fn new(max_tasks: usize) -> Self {
        RelationshipGraph {
            relationships: [[1; 16]; 16], // Default neutral-friendly relationship
            size: max_tasks.min(16),
        }
    }

    /// Set relationship strength between two tasks
    pub fn set_relationship(&mut self, from: usize, to: usize, strength: i8) {
        if from < self.size && to < self.size {
            self.relationships[from][to] = strength;
        }
    }

    /// Get relationship strength from one task to another
    pub fn get_relationship(&self, from: usize, to: usize) -> i8 {
        if from < self.size && to < self.size {
            self.relationships[from][to]
        } else {
            0
        }
    }

    /// Get relationship strength between two task IDs (convenience)
    pub fn get_relationship_by_id(&self, tasks: &[Task], from_id: u64, to_id: u64) -> i8 {
        let from_idx = tasks.iter().position(|t| t.id == from_id);
        let to_idx = tasks.iter().position(|t| t.id == to_id);

        match (from_idx, to_idx) {
            (Some(f), Some(t)) => self.get_relationship(f, t),
            _ => 0,
        }
    }
}

pub struct SwarmRuntime {
    scheduler: SelfImprovingScheduler,
    tasks: [Task; 16],
    task_count: usize,
    relationships: RelationshipGraph,
}

impl SwarmRuntime {
    pub const fn new() -> Self {
        SwarmRuntime {
            scheduler: SelfImprovingScheduler::new(),
            tasks: [Task {
                id: 0,
                priority: 0,
                emotional_state: EmotionalState::Neutral,
                memory: [None; 4],
                memory_index: 0,
            }; 16],
            task_count: 0,
            relationships: RelationshipGraph::new(16),
        }
    }

    pub fn add_task(&mut self, base_priority: u8) -> Option<usize> {
        if self.task_count >= self.tasks.len() {
            return None;
        }

        let task = self.scheduler.create_task(base_priority);
        let index = self.task_count;
        self.tasks[index] = task;
        self.task_count += 1;

        Some(index)
    }

    pub fn get_task(&self, index: usize) -> Option<&Task> {
        if index < self.task_count {
            Some(&self.tasks[index])
        } else {
            None
        }
    }

    pub fn get_task_mut(&mut self, index: usize) -> Option<&mut Task> {
        if index < self.task_count {
            Some(&mut self.tasks[index])
        } else {
            None
        }
    }

    /// Set relationship strength between two tasks (by index)
    pub fn set_relationship(&mut self, from: usize, to: usize, strength: i8) {
        self.relationships.set_relationship(from, to, strength);
    }

    /// Run emotional propagation across the swarm with multi-hop support
    pub fn propagate_emotions(&mut self, max_hops: usize) {
        if self.task_count == 0 {
            return;
        }

        for i in 0..self.task_count {
            let source = self.tasks[i];

            // Only propagate from tasks with meaningful emotional states
            if source.emotional_state.is_positive() || source.emotional_state == EmotionalState::Stressed {
                // Use the relationship-aware multi-hop propagation
                // For now we call the scheduler's method (it uses default relationships)
                // In a more advanced version we would pass the graph
                let mut targets = self.tasks;
                self.scheduler.propagate_emotion_multi_hop(&source, &mut targets, max_hops);

                // Copy changes back
                for j in 0..self.task_count {
                    self.tasks[j] = targets[j];
                }
            }
        }
    }

    pub fn run_scheduling(&mut self) {
        let mut active_tasks = [Task {
            id: 0,
            priority: 0,
            emotional_state: EmotionalState::Neutral,
            memory: [None; 4],
            memory_index: 0,
        }; 16];

        for i in 0..self.task_count {
            active_tasks[i] = self.tasks[i];
        }

        self.scheduler.schedule(&mut active_tasks[0..self.task_count]);

        for i in 0..self.task_count {
            self.tasks[i] = active_tasks[i];
        }
    }

    pub fn print_swarm_state(&self) {
        println!("Swarm state ({} tasks):", self.task_count);
        for i in 0..self.task_count {
            let t = &self.tasks[i];
            println!("  Task {}: priority={}, state={:?}", t.id, t.priority, t.emotional_state);
        }
    }
}

// TODO:
// - Make propagate_emotions use the RelationshipGraph for dynamic strength
// - Add methods to query emotional memory across the swarm
// - Implement more sophisticated propagation strategies
