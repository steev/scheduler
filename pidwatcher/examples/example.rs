extern crate pidwatcher;

use pidwatcher::PidWatcher;
use std::{ffi::OsStr, thread, time::Duration};

fn main() {
    let mut watcher = PidWatcher::default();
    loop {
        watcher.scan(|processes| {
            for process in processes {
                if let Ok(exe) = process.exe() {
                    if let Some(name) = exe.file_name().and_then(OsStr::to_str) {
                        println!("{}", name);
                    }
                }
            }
        });

        thread::sleep(Duration::new(1, 0));
    }
}
