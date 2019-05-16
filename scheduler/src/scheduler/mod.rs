use crate::{Id, Process, ProcessGroup, User};
use bitvec::vec::BitVec;
use core::{convert::TryFrom, mem};
use libc::c_ulong;

mod errors;
mod parameters;
mod policy;

// pub use self::cpuset::*;
pub use self::{errors::*, parameters::*, policy::*};
use bitvec::prelude::LittleEndian;

pub trait Scheduling: Copy + Into<libc::pid_t> {
    fn get_affinity(&self) -> Result<BitVec<LittleEndian, c_ulong>, SchedulerError> {
        let mut set = bitvec![LittleEndian, c_ulong; 0; 2048];

        let result = unsafe {
            let slice: &mut [c_ulong] = set.as_mut();
            libc::syscall(
                libc::SYS_sched_getaffinity,
                (*self).into(),
                mem::size_of::<c_ulong>(),
                slice.as_mut_ptr(),
            )
        };

        match result {
            -1 => Err(SchedulerError::from_errno()),
            _ => Ok(set),
        }
    }

    fn get_parameters(&self) -> Result<Parameters, SchedulerError> {
        let mut parameters = Parameters::default();

        match unsafe {
            libc::sched_getparam((*self).into(), &mut parameters as &mut libc::sched_param)
        } {
            -1 => Err(SchedulerError::from_errno()),
            _ => Ok(parameters),
        }
    }

    fn get_scheduler(&self) -> Result<Policy, SchedulerError> {
        match unsafe { libc::sched_getscheduler((*self).into()) } {
            -1 => Err(SchedulerError::from_errno()),
            other => {
                Policy::try_from(other as u8).map_err(|_| SchedulerError::UnknownPolicy(other))
            }
        }
    }

    fn set_affinity(&self, cpuset: &[c_ulong]) -> Result<(), SchedulerError> {
        let result = unsafe {
            libc::syscall(
                libc::SYS_sched_setaffinity,
                (*self).into(),
                mem::size_of::<libc::c_ulong>(),
                cpuset.as_ptr(),
            )
        };

        match result {
            -1 => Err(SchedulerError::from_errno()),
            _ => Ok(()),
        }
    }

    fn set_parameters(&self, parameters: Parameters) -> Result<(), SchedulerError> {
        match unsafe { libc::sched_setparam((*self).into(), &parameters as &libc::sched_param) } {
            -1 => Err(SchedulerError::from_errno()),
            _ => Ok(()),
        }
    }

    fn set_scheduler(&self, policy: Policy, params: Parameters) -> Result<(), SchedulerError> {
        let result = unsafe {
            libc::sched_setscheduler(
                (*self).into(),
                policy as libc::c_int,
                &params as &libc::sched_param,
            )
        };

        match result {
            -1 => Err(SchedulerError::from_errno()),
            _ => Ok(()),
        }
    }
}

impl Scheduling for Process {}
impl Scheduling for ProcessGroup {}
impl Scheduling for User {}
impl Scheduling for Id {}
