use errno::errno;

#[derive(Debug)]
pub enum SchedulerError {
    InvalidArgs,
    Permissions,
    NotFound,
    Other,
    UnknownPolicy(libc::c_int),
}

impl SchedulerError {
    pub(crate) fn from_errno() -> Self {
        match errno().into() {
            libc::EINVAL => SchedulerError::InvalidArgs,
            libc::EPERM => SchedulerError::Permissions,
            libc::ESRCH => SchedulerError::NotFound,
            _ => SchedulerError::Other,
        }
    }
}
