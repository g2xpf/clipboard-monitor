extern crate clipboard;

pub mod types;

use clipboard::x11_clipboard::{Selection, X11ClipboardContext};
use clipboard::ClipboardProvider;
use std::error::Error;
use std::io::{self, Write};
use std::thread;
use std::thread::JoinHandle;
use std::time;

pub struct ClipboardMonitor<S: Selection> {
    interval: time::Duration,
    ctx: X11ClipboardContext<S>,
    content: String,
}

impl<S> ClipboardMonitor<S>
where
    S: Selection + Send + 'static,
{
    fn new(interval: time::Duration) -> Result<Self, Box<dyn Error>> {
        Ok(ClipboardMonitor {
            interval,
            ctx: ClipboardProvider::new()?,
            content: String::new(),
        })
    }

    pub fn monitor<F>(
        interval: time::Duration,
        mut callback: F,
    ) -> Result<JoinHandle<()>, Box<dyn Error>>
    where
        F: FnMut(&str) + Send + 'static,
    {
        let mut monitor = Self::new(interval)?;

        let handle = thread::spawn(move || loop {
            let current_content: String = match monitor.ctx.get_contents() {
                Ok(content) => content,
                Err(err) => {
                    let stdout = io::stdout();
                    writeln!(&mut stdout.lock(), "{:?}", err).unwrap();
                    continue;
                }
            };

            if monitor.content != current_content {
                callback(&current_content);
                monitor.content = current_content;
            }

            thread::sleep(monitor.interval);
        });

        Ok(handle)
    }
}
