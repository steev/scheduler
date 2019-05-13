use libc::pid_t;
use procfs::{self, Process};
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum PidEvent {
    Created,
    Deleted,
}

#[derive(Default)]
pub struct PidWatcher {
    seen:      HashSet<pid_t>,
    missing:   Vec<pid_t>,
    processes: Vec<Process>,
}

impl PidWatcher {
    pub fn scan<T, F: FnMut(&[Process]) -> T>(&mut self, mut func: F) -> T {
        self.processes.clear();
        let processes = procfs::all_processes();

        // Check for missing PIDs.
        for value in &self.seen {
            if !self.seen.contains(value) {
                self.missing.push(*value);
            }
        }

        // Remove the missing PIDs.
        for value in &self.missing {
            self.seen.remove(value);
        }

        // Check for new PIDs
        for process in processes {
            if self.seen.insert(process.pid()) {
                self.processes.push(process)
            }
        }

        self.missing.clear();
        func(&self.processes)
    }
}
