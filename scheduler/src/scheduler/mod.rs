use crate::{Id, Process, ProcessGroup, User};
use core::{convert::TryFrom, mem};

mod cpuset;
mod errors;
mod parameters;
mod policy;

pub use self::{cpuset::*, errors::*, parameters::*, policy::*};

pub trait Scheduling: Copy + Into<libc::pid_t> {
    fn get_affinity(&self) -> Result<CpuSet, SchedulerError> {
        let mut cpuset = CpuSet::default();
        let result = unsafe {
            libc::sched_getaffinity(
                (*self).into(),
                mem::size_of::<libc::cpu_set_t>() as libc::size_t,
                &mut cpuset as &mut libc::cpu_set_t,
            )
        };

        match result {
            -1 => Err(SchedulerError::from_errno()),
            _ => Ok(cpuset),
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

    fn set_affinity(&self, cpuset: &CpuSet) -> Result<(), SchedulerError> {
        let result = unsafe {
            libc::sched_setaffinity(
                (*self).into(),
                mem::size_of::<libc::cpu_set_t>() as libc::size_t,
                &cpuset as &libc::cpu_set_t,
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
