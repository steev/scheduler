use crate::{Id, Process, ProcessGroup, User};

pub trait CpuPriority: Copy + Into<libc::pid_t> {
    fn what(&self) -> u32;

    fn get_priority(&self) -> libc::c_int {
        unsafe { libc::getpriority(self.what(), (*self).into() as libc::id_t) }
    }

    fn set_priority(&self, priority: libc::c_int) -> libc::c_int {
        unsafe { libc::setpriority(self.what(), (*self).into() as libc::id_t, priority) }
    }
}

impl CpuPriority for Process {
    fn what(&self) -> u32 { 0 }
}

impl CpuPriority for ProcessGroup {
    fn what(&self) -> u32 { 1 }
}

impl CpuPriority for User {
    fn what(&self) -> u32 { 2 }
}

impl CpuPriority for Id {
    fn what(&self) -> u32 {
        match self {
            Id::Process(id) => id.what(),
            Id::ProcessGroup(id) => id.what(),
            Id::User(id) => id.what(),
        }
    }
}
