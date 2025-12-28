use std::{thread, time::Duration};

use anyhow::Result;
use x11_clipboard::Clipboard;

use crate::listener::ClipboardListener;

pub struct X11ClipboardListener<F>
where
    F: Fn(String) + Send + 'static,
{
    clipboard: Clipboard,
    on_change: F,
    last_value: Option<String>,
}

impl<F> X11ClipboardListener<F>
where
    F: Fn(String) + Send + 'static,
{
    pub fn new(on_change: F) -> Result<Self> {
        Ok(Self {
            clipboard: Clipboard::new()?,
            on_change,
            last_value: None,
        })
    }
}

impl<F> ClipboardListener for X11ClipboardListener<F>
where
    F: Fn(String) + Send + 'static,
{
    fn start(&mut self) -> Result<()> {
        loop {
            if let Ok(text) = self.clipboard.load(
                self.clipboard.getter.atoms.clipboard,
                self.clipboard.getter.atoms.utf8_string,
                self.clipboard.getter.atoms.property,
                Duration::from_millis(100),
            ) {
                if let Ok(text_sting) = String::from_utf8(text) {
                    if self.last_value.as_ref() != Some(&text_sting) {
                        self.last_value = Some(text_sting.clone());
                        (self.on_change)(text_sting);
                    }
                }
            }

            thread::sleep(Duration::from_millis(300));
        }
    }
}
