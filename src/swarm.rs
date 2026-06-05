/// SwarmRuntime for Nova OS
///
/// Higher-level abstraction for managing groups of emotional tasks/agents.
/// This is where swarm-level emotional intelligence lives.

use crate::scheduler::{SelfImprovingScheduler, Task, EmotionalState};

pub struct SwarmRuntime {
    scheduler: SelfImprovingScheduler,
    tasks: [Task; 16], // Fixed size for early kernel (no alloc)
    task_count: usize,
}

impl SwarmRuntime {
    pub const fn new() -> Self {
        SwarmRuntime {
            scheduler: SelfImprovingScheduler::new(),
            tasks: [Task {
                id: 0,
                priority: 0,
                emotional_state: EmotionalState::Neutral,
            }; 16],
            task_count: 0,
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

    /// Run emotional propagation across the entire swarm with multiple hops
    pub fn propagate_emotions(&mut self, max_hops: usize) {
        if self.task_count == 0 {
            return;
        }

        // Simple approach: propagate from every task that has a strong emotional state
        for i in 0..self.task_count {
            let source = self.tasks[i];
            if source.emotional_state.is_positive() || source.emotional_state == EmotionalState::Stressed {
                // Propagate to all other tasks
                let mut targets: [Task; 16] = self.tasks;
                self.scheduler.propagate_emotion_multi_hop(&source, &mut targets, max_hops);

                // Copy back changes
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
// - Replace fixed array with dynamic collection when alloc is stable
// - Add real relationship graph
// - Add emotional memory per task
// - Better propagation strategy (not every positive task propagates every time)
