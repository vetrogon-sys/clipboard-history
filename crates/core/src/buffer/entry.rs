use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct ClipboardEntry {
    pub text: String,
    pub timestamp: SystemTime,
}

impl ClipboardEntry {
    pub fn new(text: String) -> Self {
        Self {
            text,
            timestamp: SystemTime::now(),
        }
    }
}
