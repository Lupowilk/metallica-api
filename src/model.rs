use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Song {
    pub title: String,
    pub album: String,
    pub year: u16,
    pub length_seconds: u32,
}

