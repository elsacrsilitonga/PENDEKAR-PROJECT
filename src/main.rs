use actix_web::{HttpServer, App, web, middleware};
use sqlx::postgres::PgPoolOptions;

mod app;
mod handler;

#[actix_web::main]
async fn main() -> Result<(), app::error::CustomError> { 
    app::init_logger();

    // mempersiapkan/membuat koneksi ke database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(env!("DATABASE_URL")).await?;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))

            .service( handler::services() )
        })
        .bind("127.0.0.1:8090")?
        .run()
        .await?;

    Ok(())
}