use actix_web::{App, HttpServer, web};
//use sqlx::postgres::PgPoolOptions;
//use std::env;
use dotenv::dotenv;
use db::create_pool;

mod models;
mod routes;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Get the database URL from the environment variable or default it
    let pool = create_pool().await;
    

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Share the pool across handlers
            .configure(|cfg| {
                cfg.service(routes::add_item);
                cfg.service(routes::get_items);
                cfg.service(routes::update_item);
                cfg.service(routes::delete_item);
            })
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
