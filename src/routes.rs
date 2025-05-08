use crate::model::Song;
use crate::data::SongDatabase;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};


// Create Handler Functions

#[get("/songs")]
async fn get_songs(db: web::Data<SongDatabase>) -> impl Responder {
    let songs = db.get_all_songs(); // retrieves the full list of songs from your database
    HttpResponse::Ok().json(songs) // returns the list of songs as JSON
}