mod model;
mod routes;
mod data;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use crate::data::SongDatabase;
use crate::routes::config_routes;

fn main() {
    println!("Hello, world!");
}
