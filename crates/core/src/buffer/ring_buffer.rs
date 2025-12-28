use std::collections::VecDeque;
use std::path::Path;
use anyhow::Result;

use super::ClipboardEntry;
use crate::persistence;

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

    /// Create a new buffer and load entries from persistence file if it exists
    pub fn new_with_persistence<P: AsRef<Path>>(max_entries: usize, path: P) -> Result<Self> {
        let mut buffer = Self::new(max_entries);
        buffer.load_from_file(path)?;
        Ok(buffer)
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

    /// Get all entries as a Vec (for serialization)
    pub fn entries_vec(&self) -> Vec<ClipboardEntry> {
        self.entries.iter().cloned().collect()
    }

    /// Get the number of entries in the buffer
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Save entries to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let entries_vec = self.entries_vec();
        persistence::save_to_file(path, &entries_vec)
    }

    /// Load entries from a file
    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let loaded_entries = persistence::load_from_file(path)?;

        // Clear existing and load persisted entries
        self.entries.clear();
        for entry in loaded_entries.into_iter().take(self.max_entries) {
            self.entries.push_back(entry);
        }

        Ok(())
    }
}
