use crate::model::Song;
use crate::data::SongDatabase;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};


// Create Handler Functions
#[get("/songs")]
async fn get_songs(db: web::Data<SongDatabase>) -> impl Responder {
    let songs = db.get_all_songs(); // retrieves the full list of songs from your database
    HttpResponse::Ok().json(songs) // returns the list of songs as JSON
}


// Add a handler for finding specific songs by title
#[get("/songs/{title}")]
async fn get_song_by_title(db: web::Data<SongDatabase>, title: web::Path<String>) -> impl Responder {
    let song_title = title.into_inner(); // get the actual string from the parameter
    let found_song = db.find_by_title(&song_title); // search for the song
    
    if let Some(song) = found_song { 
        HttpResponse::Ok().json(song)
    } else {
        HttpResponse::NotFound().body("Song not found")
    }
}