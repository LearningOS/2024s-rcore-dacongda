//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The start time
    pub start_time: usize,
    /// The system call times
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
}

/// Task information
#[derive(Copy, Clone)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

impl TaskInfo {
    /// New a task
    pub fn new(status: TaskStatus) -> Self {
        let syscall_times = [0; MAX_SYSCALL_NUM];
        TaskInfo {
            status,
            syscall_times,
            time: 0
        }
    }

    /// Zero init a task
    pub fn zero_init() -> Self {
        Self {
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0
        }
    }

    /// get status
    pub fn get_status(&self) -> TaskStatus {
        self.status
    }

    /// get syscall_times
    pub fn get_syscall_times(&self) -> [u32; MAX_SYSCALL_NUM] {
        self.syscall_times
    }

    /// get time
    pub fn get_time(&self) -> usize {
        self.time
    }

    /// set status
    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    /// add syscall_time
    pub fn add_syscall_time(&mut self, index: usize) {
        if index < MAX_SYSCALL_NUM {
            self.syscall_times[index] += 1;
        }
    }

    /// set time
    pub fn set_time(&mut self, time: usize) {
        self.time = time
    }

    /// set syscall times
    pub fn set_syscall_times(&mut self, syscall_times: [u32; MAX_SYSCALL_NUM]) {
        self.syscall_times = syscall_times
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
