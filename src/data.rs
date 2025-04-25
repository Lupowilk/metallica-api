use std::sync::Mutex;
use crate::model::Song;

pub struct SongDatabase {
    songs: Mutex<Vec<Song>>,
}

impl SongDatabase {
    // Create a new empty database
    pub fn new() -> Self {
        SongDatabase {
            songs: Mutex::new(Vec::new()),
        }
    }

    // Initialize with sample data

}