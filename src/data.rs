use std::sync::Mutex;
use crate::model::Song;

pub struct SongDatabase {
    songs: Mutex<Vec<Song>>,
}

impl SongDatabase {
    // Create a new empty database
    pub fn new_db() -> Self {
        SongDatabase {
            songs: Mutex::new(Vec::new()),
        }
    }

    // Initialize with sample data
    pub fn init_sample_db() -> Self {
        let mut db = SongDatabase::new_db();
        
        let song1 = Song {
            title: String::from("Enter Sandman"),
            album: String::from("Metallica"),
            year: 1991,
            length_seconds: 332,
        };
        

        let song2 = Song {
            title: String::from("Inamorata"),
            album: String::from("72 Seasons"),
            year: 2022,
            length_seconds: 521,
        };

        let song3 = Song {
            title: String::from("Master of Puppets"),
            album: String::from("Master of Puppets"),
            year: 1986,
            length_seconds: 515,
        };
        
        let song4 = Song {
            title: String::from("One"),
            album: String::from("...And Justice for All"),
            year: 1988,
            length_seconds: 447,
        };

        let song5 = Song {
            title: String::from("Nothing Else Matters"),
            album: String::from("Metallica"),
            year: 1991,
            length_seconds: 386,
        };

        let song6 = Song {
            title: String::from("For Whom the Bell Tolls"),
            album: String::from("Ride the Lightning"),
            year: 1984,
            length_seconds: 310,
        };
        
        {
        let mut songs_lock = db.songs.lock().unwrap();
            songs_lock.push(song1);
            songs_lock.push(song2);
            songs_lock.push(song3);
            songs_lock.push(song4);
            songs_lock.push(song5);
            songs_lock.push(song6);
        }; // Release the song_lock borrow
        
        db
    }
    
    pub fn get_all_songs(&self) -> Vec<Song> {
        let songs = self.songs.lock().unwrap();
        songs.clone() // returns a new Vec<Song> with cloned songs
    }
    
    pub fn find_by_title(&self, title: &str) -> Option<Song> {
        
    }
}