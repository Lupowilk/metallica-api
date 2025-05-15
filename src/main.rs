mod model;
mod routes;
mod data;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use crate::data::SongDatabase;
use crate::routes::config_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    // Create a database with sample songs
    let db = SongDatabase::init_sample_db();

    // Wrap the database in a Data instance so it can be shared between handlers
    let app_data = web::Data::new(db);

    // Create and configure the HTTP server
    HttpServer::new(move || {
        App::new() // Create a new App instance for each worker
            .app_data(app_data.clone())// Register the shared database
            .configure(config_routes) // Configure all our routes
    })
        .bind("127.0.0.1:8080")?// Bind to a port and address
        .run() // Run the server
        .await // Wait until the server is done
}
