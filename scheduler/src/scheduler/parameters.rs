use core::ops::{Deref, DerefMut};

#[derive(Clone, Copy)]
pub struct Parameters(libc::sched_param);

impl Default for Parameters {
    fn default() -> Self { Parameters(libc::sched_param { sched_priority: 0 }) }
}

impl Deref for Parameters {
    type Target = libc::sched_param;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Parameters {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
