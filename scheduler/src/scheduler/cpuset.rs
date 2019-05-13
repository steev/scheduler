use super::SchedulerError;
use core::{
    mem,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Copy)]
pub struct CpuSet(libc::cpu_set_t);

impl CpuSet {
    pub fn is_set(&self, field: usize) -> Result<bool, SchedulerError> {
        Self::validate(field)?;
        Ok(unsafe { libc::CPU_ISSET(field, &self.0) })
    }

    pub fn clear(&mut self) -> Result<(), SchedulerError> {
        unsafe { libc::CPU_ZERO(&mut self.0) }
        Ok(())
    }

    pub fn set(&mut self, field: usize) -> Result<(), SchedulerError> {
        Self::validate(field)?;
        unsafe { libc::CPU_SET(field, &mut self.0) }
        Ok(())
    }

    pub fn unset(&mut self, field: usize) -> Result<(), SchedulerError> {
        Self::validate(field)?;
        unsafe { libc::CPU_CLR(field, &mut self.0) };
        Ok(())
    }

    fn validate(field: usize) -> Result<(), SchedulerError> {
        if field >= 8 * mem::size_of::<libc::cpu_set_t>() {
            Err(SchedulerError::InvalidArgs)
        } else {
            Ok(())
        }
    }
}

impl Default for CpuSet {
    fn default() -> CpuSet { CpuSet(unsafe { mem::zeroed() }) }
}

impl Deref for CpuSet {
    type Target = libc::cpu_set_t;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for CpuSet {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
