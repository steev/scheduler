#![no_std]

pub use self::{priority::*, scheduler::*};

pub mod priority;
pub mod scheduler;

macro_rules! idtype {
    ($name:ident) => {
        #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
        pub struct $name(libc::pid_t);

        impl $name {
            pub fn new(id: u64) -> Self {
                $name(id as libc::pid_t)
            }

            pub fn current() -> Self {
                Self(0)
            }
        }

        impl From<$name> for libc::pid_t {
            fn from(id: $name) -> Self {
                id.0
            }
        }
    };
}

idtype!(Process);
idtype!(ProcessGroup);
idtype!(User);

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Id {
    Process(Process),
    ProcessGroup(ProcessGroup),
    User(User),
}

impl From<Id> for libc::pid_t {
    fn from(id: Id) -> Self {
        match id {
            Id::Process(id) => id.into(),
            Id::ProcessGroup(id) => id.into(),
            Id::User(id) => id.into(),
        }
    }
}
