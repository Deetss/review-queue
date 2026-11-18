//! Opt-in input trace for chasing Orca focus bugs. `touch ~/.claude/review/events.log` turns
//! it on; deleting the file turns it off. A freeze with events still arriving means Orca stopped
//! repainting the pane; a freeze with nothing arriving means Orca stopped sending input.

use crossterm::event::{Event, MouseEventKind};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub struct EventLog {
    file: Option<File>,
}

impl EventLog {
    /// Appends to the trace only if it already exists, so nothing is created unasked.
    pub fn open() -> Self {
        let path = PathBuf::from(std::env::var("HOME").unwrap_or_default())
            .join(".claude/review/events.log");
        let file = path
            .exists()
            .then(|| OpenOptions::new().append(true).open(&path).ok())
            .flatten();
        Self { file }
    }

    pub fn enabled(&self) -> bool {
        self.file.is_some()
    }

    pub fn record(&mut self, ev: &Event) {
        let Some(file) = self.file.as_mut() else {
            return;
        };
        let line = match ev {
            Event::Mouse(m) if m.kind == MouseEventKind::Moved => {
                format!("move {},{}", m.column, m.row)
            }
            other => format!("{other:?}"),
        };
        let ts = chrono::Local::now().format("%H:%M:%S%.3f");
        let _ = writeln!(file, "{ts} pid={} {line}", std::process::id());
    }
}
