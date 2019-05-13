use core::{convert::TryFrom, str::FromStr};
use libc::{SCHED_BATCH, SCHED_FIFO, SCHED_IDLE, SCHED_OTHER, SCHED_RR};

const SCHED_DEADLINE: libc::c_int = 6;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Policy {
    Other = SCHED_OTHER as u8,
    Fifo = SCHED_FIFO as u8,
    Rr = SCHED_RR as u8,
    Batch = SCHED_BATCH as u8,
    Idle = SCHED_IDLE as u8,
    Deadline = SCHED_DEADLINE as u8,
}

impl TryFrom<u8> for Policy {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match libc::c_int::from(value) {
            SCHED_OTHER => Ok(Policy::Other),
            SCHED_FIFO => Ok(Policy::Fifo),
            SCHED_RR => Ok(Policy::Rr),
            SCHED_BATCH => Ok(Policy::Batch),
            SCHED_IDLE => Ok(Policy::Idle),
            SCHED_DEADLINE => Ok(Policy::Deadline),
            _ => Err(()),
        }
    }
}

impl FromStr for Policy {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let policy = match input {
            "other" => Policy::Other,
            "fifo" => Policy::Fifo,
            "rr" => Policy::Rr,
            "batch" => Policy::Batch,
            "idle" => Policy::Idle,
            "deadline" => Policy::Deadline,
            _ => return Err(()),
        };

        Ok(policy)
    }
}

impl From<Policy> for &'static str {
    fn from(policy: Policy) -> Self {
        match policy {
            Policy::Other => "other",
            Policy::Fifo => "fifo",
            Policy::Rr => "rr",
            Policy::Batch => "batch",
            Policy::Idle => "idle",
            Policy::Deadline => "deadline",
        }
    }
}
