use std::collections::VecDeque;

use super::ClipboardEntry;

pub struct ClipboardBuffer {
    max_entries: usize,
    entries: VecDeque<ClipboardEntry>,
}

impl ClipboardBuffer {
    pub fn new(max_entries: usize) -> Self {
        Self {
            max_entries,
            entries: VecDeque::with_capacity(max_entries),
        }
    }

    pub fn push(&mut self, entry: ClipboardEntry) {
        // Deduplicate exact text
        if let Some(pos) = self.entries.iter().position(|e| e.text == entry.text) {
            self.entries.remove(pos);
        }

        self.entries.push_front(entry);

        if self.entries.len() > self.max_entries {
            self.entries.pop_back();
        }
    }

    pub fn entries(&self) -> impl Iterator<Item = &ClipboardEntry> {
        self.entries.iter()
    }
}
