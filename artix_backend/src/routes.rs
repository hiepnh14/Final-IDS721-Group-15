use actix_web::{web};
use crate::handlers::{get_messages, post_message};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(get_messages))
            .route(web::post().to(post_message))
    );
}
