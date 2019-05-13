# Process Scheduler Daemon

A process scheduling daemon for unix systems, with a TOML configuration format, written in Rust, and tested on Linux.

## Features

This is a complete list of features that have been, or are planned to be, implemented.

- [x] A simple TOML configuration file for controlling the behavior of the daemon
- [ ] Reloads the configuration when the configuration file has been modified
- [x] Monitors PIDs and renices them shortly after they are created.
- [x] Rules are defined with regular expressions
- [x] Rules are applied either by the name or owner of the new process
- [x] Rules may define CPU and I/O priorities
- [x] Rules may define scheduling policies
- [ ] Rules may define CPU affinities
- [ ] Rules may limit CPU usage
- [ ] Supports IPC communication with frontends
- [ ] GTK frontend
- [ ] OrbTk frontend


## TOML Example

```toml
# Unimportant users
avahi = { by = "owner", priority = 20 }

# Important users
gdm = { by = "owner", priority = -10 }

# Desktop applications
"^(atom|code|firefox|mpv)" = { priority = -5 }

# Desktop-critical services
"^(gdm|gnome-shell|Xorg)" = { priority = -10 }

# Compiler tasks
"^(make|cargo|rustc|rls|cicc|gcc)" = { priority = 20, policy = "idle" }

# Non-critical background services
"^(tracker|system76-|io.elementary.appcenter)" = { priority = 20, policy = "idle" }

# Realtime services
pulseaudio = { priority = -15, policy = "deadline" }
```
