use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use sysinfo::{System, ProcessRefreshKind, RefreshKind};
use device_query::{DeviceQuery, DeviceState};

pub enum Event {
    Tick,
    DistractionDetected,
    FocusPulse,
}

pub fn start_watcher(tx: mpsc::Sender<Event>, blacklist: Vec<String>) {
    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::new())
    );
    let device_state = DeviceState::new();

    thread::spawn(move || {
        loop {
            sys.refresh_processes();

            let distracted = sys.processes().values().any(|p| {
                let name = p.name().to_lowercase();
                blacklist.iter().any(|b| name.contains(b))
            });

            if distracted {
                let _ = tx.send(Event::DistractionDetected);
            }

            if !device_state.get_keys().is_empty() {
                let _ = tx.send(Event::FocusPulse);
            }

            let _ = tx.send(Event::Tick);
            thread::sleep(Duration::from_millis(1000));
        }
    });
}