use crate::model::Song;
use crate::data::SongDatabase;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};


// Create Handler Functions

#[get("/songs")]
async fn get_songs(db: web::<SongDatabase>) -> impl Responder {
    
}