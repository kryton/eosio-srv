use actix_web::{web, App, HttpServer};

mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handlers::index))
            .route("/again", web::get().to(handlers::index2))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
