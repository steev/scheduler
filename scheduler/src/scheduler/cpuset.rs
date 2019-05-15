use core::ptr::NonNull;
use libc::{c_int, cpu_set_t, malloc, CPU_CLR, CPU_ISSET, CPU_SET, CPU_ZERO};

pub struct CpuSet(NonNull<cpu_set_t>);

impl CpuSet {
    pub fn as_mut(&mut self) -> *mut cpu_set_t {
        self.0.as_ptr()
    }

    pub fn as_ptr(&self) -> *const cpu_set_t {
        self.0.as_ptr()
    }

    pub fn new(num_cpus: u32) -> Self {
        let ptr = unsafe {
            let ptr = CPU_ALLOC(num_cpus as i32);
            CPU_ZERO(ptr);
            ptr
        };

        Self(NonNull::new(ptr).expect("cpu_set_t pointer was null"))
    }

    pub fn count(&self) -> usize {
        unsafe { CPU_COUNT(self.0.as_ptr()) as usize }
    }

    pub fn is_set(&self, field: u32) -> bool {
        unsafe { CPU_ISSET(field as i32, self.0.as_ptr()) }
    }

    pub fn set(&mut self, field: u32) {
        unsafe {
            CPU_SET(field as i32, self.0.as_ptr());
        }
    }

    pub fn unset(&mut self, field: u32) {
        unsafe {
            CPU_CLR(field as i32, self.0.as_ptr());
        }
    }
}

impl Default for CpuSet {
    fn default() -> Self {
        Self::new(2048)
    }
}

impl Drop for CpuSet {
    fn drop(&mut self) {
        unsafe {
            CPU_FREE(self.0.as_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CpuSet;

    #[test]
    fn cpu_set() {
        let mut set = CpuSet::default();
        assert!(!set.is_set(0));

        set.set(0);
        assert!(set.is_set(0))
    }
}
