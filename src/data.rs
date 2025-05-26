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
        let songs = self.songs.lock().unwrap();
        songs.iter().find(|song| {
            song.title.to_lowercase() == title.to_lowercase()
        }).cloned()
    }
    
    //A method to get all metallica songs in the API
    pub fn get_all_albums(&self) -> Vec<String> {
        let songs = self.songs.lock().unwrap();
        
        let mut albums: Vec<String> = songs.iter()
            .map(|song| song.album.clone())
            .collect();
        
        albums.sort();
        
        albums.dedup();
        
        albums
    }
    
    //A method that can find all songs from a specific album
    pub fn get_songs_by_album(&self, album: &str) -> Vec<Song> {
        let songs = self.songs.lock().unwrap();
        let album_songs: Vec<Song> = songs.iter()
            .filter( |song| song.album.to_lowercase() == album.to_lowercase())
            .cloned()
            .collect();
        
        album_songs
    } 
    
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_db() {
        // test creating a new empty database
        let db = SongDatabase::new_db();
        let songs = db.get_all_songs();
        assert_eq!(songs.len(), 0);
    }
    
    #[test]
    fn test_init_sample_db() {
        // test creating a database with sample data
        let db = SongDatabase::init_sample_db();
        let songs = db.get_all_songs();
        assert_eq!(songs.len(), 6);
    }
    
    #[test]
    fn test_find_by_title_existing_song() {
        //Test finding a song that exists
        let db = SongDatabase::init_sample_db();
        let song = db.find_by_title("Enter Sandman");
        assert!(song.is_some());
        let found_song =song.unwrap();
        assert_eq!(found_song.title, "Enter Sandman");
        assert_eq!(found_song.album, "Metallica");
    }

    #[test]
    fn test_get_all_albums() {
        // Test getting all unique album names
        let db = SongDatabase::init_sample_db();
        let albums = db.get_all_albums();

        // We should have 5 unique albums in our sample data
        assert_eq!(albums.len(), 5);

        // Check that albums are sorted and contain expected names
        assert!(albums.contains(&"Metallica".to_string()));
        assert!(albums.contains(&"Master of Puppets".to_string()));
    }

    #[test]
    fn test_get_songs_by_album() {
        // Test getting songs from a specific album
        let db = SongDatabase::init_sample_db();
        let metallica_songs = db.get_songs_by_album("Metallica");

        // The "Metallica" album should have 2 songs
        assert_eq!(metallica_songs.len(), 2);

        // Both songs should be from the Metallica album
        for song in metallica_songs {
            assert_eq!(song.album, "Metallica");
        }
    }
    
}