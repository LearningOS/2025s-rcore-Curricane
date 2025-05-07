//! Types related to task management

use crate::syscall::Syscall;

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,

    /// The number of syscalls made by the task
    pub syscall_count: SyscallCounts,
}

impl Default for TaskControlBlock {
    fn default() -> Self {
        Self {
            task_status: TaskStatus::UnInit,
            task_cx: TaskContext::zero_init(),
            syscall_count: SyscallCounts::default(),
        }
    }
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

/// The counts of syscalls made by a task
#[derive(Copy, Clone, Default)]
pub struct SyscallCounts {
    /// The counts of each syscall
    pub counts: [usize; 5],
}

impl SyscallCounts {
    /// Increment the count of a specific syscall
    pub fn add_one(&mut self, syscall: Syscall) {
        self.counts[syscall.idx()] += 1;
    }

    /// Get the count of a specific syscall
    pub fn get(&self, syscall: Syscall) -> usize {
        self.counts[syscall.idx()]
    }
}
