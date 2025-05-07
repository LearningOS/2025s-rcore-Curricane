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

    pub syscall_count: SyscallCounts,
}

impl TaskControlBlock {
    pub fn new(id: usize) -> Self {
        Self {
            task_status: TaskStatus::UnInit,
            task_cx: TaskContext::zero_init(),
            syscall_count: SyscallCounts,
        }
    }
}

impl Default for TaskControlBlock {
    fn default() -> Self {
        Self::new(0)
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

pub struct SyscallCounts {
    pub counts: [usize; Syscall::count()],
}

impl SyscallCounts {
    pub fn add_one(&mut self, syscall: Syscall) {
        self.counts[syscall.idx()] += 1;
    }

    pub fn get(&self, syscall: Syscall) -> usize {
        self.counts[syscall.idx()]
    }
}
