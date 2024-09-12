use actix_web::{App, HttpServer, web::Data};
use actix_cors::Cors;

mod api;
mod model;
mod repository;
mod utils;
mod prelude;
mod error;

use repository::surrealdb_repo::SurrealDBRepo;
use api::table_api::{create_table, get_tables, get_table, update_table, delete_table};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let surreal = SurrealDBRepo::init().await.expect("Error connecting to SurrealDB!");
    
    let db_data = Data::new(surreal);
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()  // Allow any origin for development purposes. Be careful with this in production.
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(db_data.clone())
            .wrap(cors)  // Add the CORS middleware before your services.
            .service(create_table)
            .service(get_tables)
            .service(get_table)
            .service(update_table)
            .service(delete_table)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
