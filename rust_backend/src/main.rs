use actix_web::{web, App, HttpServer};
mod handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(move || {
        App::new()
            .route("/us/{name}", web::get().to(handlers::get_users))
            .route("/user/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::add_user))
            .route("/users/{id}", web::delete().to(handlers::delete_user))

    })
    .bind("127.0.0.1:8010")?
    .run()
    .await
}