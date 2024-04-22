use actix_web::{App, HttpServer};
mod core;
mod handlers;
mod routes;
mod state;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(state::AppState::new())
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
