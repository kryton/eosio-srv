use actix_web::{web, App, HttpServer};

mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handlers::index))
            .service(
                web::resource("/account/{account}/action/{action_name}")
                    .route(web::get().to(handlers::action_detail))
                    .route(web::post().to(handlers::action_detail_post)),
            )
            /*
            .route(
                "/account/{account}/action/{action_name}",
                web::get().to(handlers::action_detail),
            )  .route(
                "/account/{account}/action/{action_name}",
                web::post().to(handlers::action_detail_post),
            )

             */
            .route(
                "/account/{account}/table/{table_name}",
                web::get().to(handlers::table_detail),
            )
            .route(
                "/account/{account}/rows/{scope_name}/{table_name}",
                web::get().to(handlers::table_rows),
            )
            .route(
                "/account/{account}",
                web::get().to(handlers::account_detail),
            )
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
