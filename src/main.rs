use clipboard_monitor::types::Primary;
use clipboard_monitor::ClipboardMonitor;
use std::error::Error;
use std::io::{self, Write};
use std::time;

static INTERVAL: time::Duration = time::Duration::from_millis(16);

fn main() -> Result<(), Box<dyn Error>> {
    let handle = ClipboardMonitor::<Primary>::monitor(INTERVAL, |content| {
        let escaped: String = content
            .chars()
            .filter(|c| c.is_whitespace() || c.is_ascii_alphabetic() || c.is_ascii_alphanumeric())
            .collect();
        let stdout = io::stdout();
        writeln!(&mut stdout.lock(), "{:?}", escaped).unwrap();
    })?;

    handle.join().unwrap();

    Ok(())
}
