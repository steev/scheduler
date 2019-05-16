extern crate scheduler;
use scheduler::{Process, Scheduling};

pub fn main() {
    let affinity = Process::current().get_affinity().unwrap();
    println!("ncpus: {}", affinity.count_ones());
}
