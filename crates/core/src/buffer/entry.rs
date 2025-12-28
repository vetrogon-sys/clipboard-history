use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardEntry {
    pub text: String,
    #[serde(
        serialize_with = "serialize_systemtime",
        deserialize_with = "deserialize_systemtime"
    )]
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

// Helper functions for SystemTime serialization
fn serialize_systemtime<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let duration = time
        .duration_since(UNIX_EPOCH)
        .map_err(serde::ser::Error::custom)?;
    serializer.serialize_u64(duration.as_secs())
}

fn deserialize_systemtime<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let secs = u64::deserialize(deserializer)?;
    Ok(UNIX_EPOCH + std::time::Duration::from_secs(secs))
}
