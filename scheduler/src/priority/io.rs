use crate::{Id, Process, ProcessGroup, User};

#[repr(u8)]
pub enum Class {
    None,
    Realtime,
    BestEffort,
    Idle,
}

static CLASS_STRINGS: [&str; 4] = ["none", "realtime", "best-effort", "idle"];

impl From<Class> for &'static str {
    fn from(class: Class) -> &'static str { CLASS_STRINGS[class as usize] }
}

pub trait IoPriority: Copy + Into<libc::pid_t> {
    fn what(&self) -> i64;

    fn get(&self) -> i64 {
        unsafe { libc::syscall(libc::SYS_ioprio_get, self.what(), (*self).into()) }
    }

    fn set(&self, priority: i64) -> i64 {
        unsafe { libc::syscall(libc::SYS_ioprio_set, self.what(), (*self).into(), priority) }
    }

    fn setid(&self, class: Class, level: u8) -> i64 { self.set(prio_value(class, level)) }
}

impl IoPriority for Process {
    fn what(&self) -> i64 { 1 }
}

impl IoPriority for ProcessGroup {
    fn what(&self) -> i64 { 2 }
}

impl IoPriority for User {
    fn what(&self) -> i64 { 3 }
}

impl IoPriority for Id {
    fn what(&self) -> i64 {
        match self {
            Id::Process(id) => id.what(),
            Id::ProcessGroup(id) => id.what(),
            Id::User(id) => id.what(),
        }
    }
}

// Helpers

const CLASS_SHIFT: i64 = 13;

// const PRIO_MASK: i64 = (1i64 << CLASS_SHIFT) - 1;
//
// fn prio_class(mask: i64) -> i64 {
//     mask >> CLASS_SHIFT
// }
//
// fn prio_data(mask: i64) -> i64 {
//     mask & PRIO_MASK
// }

fn prio_value(class: Class, data: u8) -> i64 { ((class as i64) << CLASS_SHIFT) | i64::from(data) }
